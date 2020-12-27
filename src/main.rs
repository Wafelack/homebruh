use clap::{App, Arg, SubCommand};
use reqwest::StatusCode;
use serde_json::Value;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn init() -> Result<(String, String), String> {
    let sources_config = "/etc/yarpm.json";
    let default_sources = "/etc/yarpm.sources";
    let binaries_path = &format!("{}/.yarpm/bin", dirs::home_dir().unwrap().to_str().unwrap());
    let path_export = &format!("export PATH=\"{}:$PATH\"", binaries_path);
    let bashrc_path = &format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap());

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

    let sources_location = if !Path::new(sources_config).exists() {
        default_sources.to_string()
    } else {
        let config_file = match std::fs::read_to_string(sources_config) {
            Ok(c) => c,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };
        let jsoned: Value = match serde_json::from_str(&config_file) {
            serde_json::Result::Ok(j) => j,
            serde_json::Result::Err(e) => return Err(format!("{} - {}", line!(), e)),
        };

        let toret = if let Value::String(s) = jsoned["sources_path"].clone() {
            s
        } else {
            default_sources.to_string()
        };
        toret.to_string()
    };

    if !Path::new(&sources_location).exists() {
        match fs::File::create(&sources_location) {
            Ok(_) => return Ok((sources_location, binaries_path.to_string())),
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        }
    }

    Ok((sources_location.to_string(), binaries_path.to_string()))
}

fn main() -> Result<(), String> {
    let (sources, binaries_path) = init()?;
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
                )
                .arg(
                    Arg::with_name("output")
                        .help("Specifies the output file")
                        .takes_value(true),
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
            matches_install.value_of("output"),
            &binaries_path,
        )?;
    }

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
fn search(sources: &String, package: &str) -> Result<Status, String> {
    let raw_sources = match fs::read_to_string(sources) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
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
    sources: &String,
    package: &str,
    output: Option<&str>,
    binaries_path: &str,
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

    if output.is_some() {
        let pathed = Path::new(output.unwrap());
        if !pathed.exists() {
            match fs::create_dir_all(pathed) {
                Ok(()) => {}
                Err(e) => return Err(format!("{} - {}", line!(), e)),
            }
        } else if pathed.exists() && pathed.is_dir() {
            return Err("Cannot use a file as output directory !".to_string());
        }
    }

    let (mut file, fname) = if output.is_some() {
        let fname = format!("{}/{}.tar.gz", output.unwrap(), package);
        let f = match fs::File::create(&fname) {
            Ok(f) => f,
            Err(e) => return Err(format!("{} - {}", line!(), e)),
        };
        (f, fname)
    } else {
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

    eprintln!("Successfully installed package `{}`", package);

    Ok(())
}
