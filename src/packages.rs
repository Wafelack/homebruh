use reqwest::StatusCode;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn upgrade(installed: &str, binaries_path: &str) -> Result<(), String> {
    let content = match fs::read_to_string(installed) {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };

    match fs::remove_dir_all(binaries_path) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    match fs::create_dir_all(binaries_path) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    let splited = content
        .split("\n")
        .filter(|l| !l.starts_with("#"))
        .filter(|l| l != &"")
        .collect::<Vec<&str>>();

    eprintln!("Upgrading {} package(s)...", splited.len());

    let mut pairs = vec![];

    for line in &splited {
        let subsplit = line.split("|").collect::<Vec<&str>>();
        if subsplit.len() != 2 {
            return Err(format!("Invalid package line : `{}`", line));
        }
        pairs.push((subsplit[0], subsplit[1]));
    }

    let mut i = 0;

    for (name, link) in pairs {
        i += 1;
        eprint!("\rUpgrading package {} out of {}", i, &splited.len());
        let rawbytes = match reqwest::blocking::get(link) {
            Ok(r) => r,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
        .bytes();
        let bytes = match rawbytes {
            Ok(b) => b.to_vec(),
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };

        let fname = &format!("{}/{}.tar.gz", binaries_path, name);

        if Path::new(fname).exists() {
            match fs::remove_file(fname) {
                Ok(()) => {}
                Err(e) => return Err(format!("{} - {}", line!(), e)),
            }
        }

        let mut f = match fs::File::create(fname) {
            Ok(f) => f,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };

        match f.write_all(&bytes) {
            Ok(()) => {}
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }

        let status = match std::process::Command::new("tar")
            .arg("-xzf")
            .arg(&fname)
            .arg("-C")
            .arg(binaries_path)
            .status()
        {
            Ok(s) => s,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };

        if !status.success() {
            return Err("Failed to extract package".to_string());
        }

        match fs::remove_file(&fname) {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
    }
    eprintln!("\n{} packages upgraded", &splited.len());

    Ok(())
}

pub enum Status {
    Found(String),
    NotFound,
}
pub fn search(sources: &str, package: &str) -> Result<Status, String> {
    let raw_sources = match fs::read_to_string(sources) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    if raw_sources.trim().is_empty() {
        return Err("No sources available !".to_string());
    }
    let sources_content = raw_sources.trim().split("\n").collect::<Vec<&str>>();

    for source in sources_content {
        let full_path = &format!("{}/{}.tar.gz", source, package);
        let resp = match reqwest::blocking::get(full_path) {
            Ok(r) => r,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };

        if resp.status().is_success() {
            return Ok(Status::Found(full_path.to_string()));
        } else if let StatusCode::NOT_FOUND = resp.status() {
            continue;
        }
    }

    Ok(Status::NotFound)
}

pub fn install(
    sources: &str,
    package: &str,
    binaries_path: &str,
    installed: &str,
) -> Result<(), String> {
    let lnk = match search(sources, package)? {
        Status::Found(s) => s,
        Status::NotFound => return Err("Package not found".to_string()),
    };

    let rawbytes = match reqwest::blocking::get(&lnk) {
        Ok(r) => r,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
    .bytes();

    let bytes = match rawbytes {
        Ok(b) => b.to_vec(),
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    let (mut file, fname) = {
        let f = match fs::File::create(&format!("{}/{}.tar.gz", binaries_path, package)) {
            Ok(f) => f,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };
        (f, format!("{}/{}.tar.gz", binaries_path, package))
    };

    match file.write_all(&bytes) {
        Ok(()) => {}
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }

    let status = match std::process::Command::new("tar")
        .arg("-xzf")
        .arg(&fname)
        .arg("-C")
        .arg(binaries_path)
        .status()
    {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    if !status.success() {
        return Err("Failed to extract package".to_string());
    }

    match fs::remove_file(&fname) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    let mut inst = match fs::OpenOptions::new().append(true).open(installed) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };

    match inst.write_all(format!("{}|{}\n", package, lnk).as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    eprintln!("Successfully installed package `{}`", package);

    Ok(())
}
