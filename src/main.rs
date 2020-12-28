use clap::{App, Arg, SubCommand};
use reqwest::StatusCode;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn init() -> Result<(String, String, String), String> {
    let installed_list = "/etc/yarpm.installed";
    let default_sources = "/etc/yarpm.sources";
    let binaries_path = &format!("{}/.yarpm/bin", dirs::home_dir().unwrap().to_str().unwrap());
    let path_export = &format!("export PATH=\"{}:$PATH\"", binaries_path);
    let bashrc_path = &format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap());

    if !Path::new(installed_list).exists() {
        match fs::File::create(installed_list) {
            Ok(_) => {}
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }

    let content = match fs::read_to_string(installed_list) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    if content.is_empty() {
        let mut f = match fs::File::create(installed_list) {
            Ok(f) => f,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };
        match f.write_all(b"# This file is not intended for manual editing\n# Manual manipulation could alterate yarpm functionning\n\n") {
            Ok(()) => {},
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }

    if !Path::new(binaries_path).exists() {
        match fs::create_dir_all(binaries_path) {
            Ok(()) => {}
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }
    let bashrc_content = match fs::read_to_string(bashrc_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    let mut bashrc = match fs::OpenOptions::new().append(true).open(bashrc_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    let mut already_written = false;

    for line in bashrc_content.split("\n").collect::<Vec<&str>>() {
        if line == path_export {
            already_written = true;
            break;
        }
    }

    if !already_written {
        match bashrc.write_all(&format!("\n{}", path_export).as_bytes()) {
            Ok(()) => {}
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }

    let sources_location = default_sources.to_string();

    if !Path::new(&sources_location).exists() {
        match fs::File::create(&sources_location) {
            Ok(_) => {
                return Ok((
                    sources_location,
                    binaries_path.to_string(),
                    installed_list.to_string(),
                ))
            }
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }

    Ok((
        sources_location.to_string(),
        binaries_path.to_string(),
        installed_list.to_string(),
    ))
}

fn main() -> Result<(), String> {
    let (sources, binaries_path, installed_list) = init()?;
    if cfg!(windows) {
        return Err("Segmentation fault\nOrigin: WIN32K_PROCESS_HANDLER 0xFAE569".to_string());
    }

    let matches = App::new("yarpm")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Yet another rusty package manager")
        .subcommand(
            SubCommand::with_name("install")
                .about("Install a package from the sources")
                .arg(
                    Arg::with_name("package")
                        .help("Specifies the package to search for")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches for a package in the sources")
                .arg(
                    Arg::with_name("package")
                        .help("Specifies the package to search for")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("sources")
                .about("Commands relative to sources management")
                .subcommand(
                    SubCommand::with_name("add").about("Add a source").arg(
                        Arg::with_name("source_link")
                            .help("Specifies the source to add")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Removes a source")
                        .arg(
                            Arg::with_name("source_link")
                                .help("Specifies the source to remove")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
        .subcommand(SubCommand::with_name("upgrade").about("Updates installed softs"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        match search(&sources, matches.value_of("package").unwrap())? {
            Status::Found(s) => println!(
                "Found a package matching `{}` at {}",
                matches.value_of("package").unwrap(),
                s
            ),
            Status::NotFound => println!("Package not found"),
        }
    } else if let Some(matches_source) = matches.subcommand_matches("sources") {
        if let Some(matches_remove) = matches_source.subcommand_matches("remove") {
            remove(matches_remove.value_of("source_link").unwrap(), &sources)?;
        } else if let Some(matches_add) = matches_source.subcommand_matches("add") {
            add(matches_add.value_of("source_link").unwrap(), &sources)?;
        }
    } else if let Some(matches_install) = matches.subcommand_matches("install") {
        install(
            &sources,
            matches_install.value_of("package").unwrap(),
            &binaries_path,
            &installed_list,
        )?;
    } else if let Some(_) = matches.subcommand_matches("upgrade") {
        upgrade(&installed_list, &binaries_path)?;
    }

    Ok(())
}

fn upgrade(installed: &str, binaries_path: &str) -> Result<(), String> {
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

fn remove(source: &str, sources_path: &str) -> Result<(), String> {
    let content = match fs::read_to_string(sources_path) {
        Ok(c) => c,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    let mut towrite = String::new();

    for line in content.replace("\n", "").split("\n").collect::<Vec<&str>>() {
        if !line.contains(source) {
            towrite.push_str(line);
            towrite.push('\n');
        }
    }

    let mut file = match fs::File::create(sources_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    match file.write_all(towrite.trim().as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }

    Ok(())
}

fn add(source: &str, sources_path: &str) -> Result<(), String> {
    let mut file_content = match fs::read_to_string(sources_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    file_content.push_str(source);
    file_content.push('\n');
    let mut file = match fs::File::create(sources_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    match file.write_all(file_content.as_bytes()) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
}

enum Status {
    Found(String),
    NotFound,
}
fn search(sources: &str, package: &str) -> Result<Status, String> {
    let raw_sources = match fs::read_to_string(sources) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    if raw_sources.trim().is_empty() {
        return Err("No sources available !".to_string());
    }
    let sources_content = raw_sources.split("\n").collect::<Vec<&str>>();

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

fn install(
    sources: &str,
    package: &str,
    binaries_path: &str,
    installed: &str,
) -> Result<(), String> {
    let lnk = match search(sources, package)? {
        Status::Found(s) => s,
        _ => return Err("Package not found".to_string()),
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
