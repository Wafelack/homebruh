use clap::{App, Arg, SubCommand};
use reqwest::StatusCode;
use serde_json::Value;
use std::fs;
use std::path::Path;

fn init() -> Result<String, String> {
    let sources_config = "/etc/yarpm.json";
    let default_sources = "/etc/yarpm.sources";

    let sources_location = if !Path::new(sources_config).exists() {
        default_sources.to_string()
    } else {
        let config_file = match std::fs::read_to_string(sources_config) {
            Ok(c) => c,
            Err(e) => return Err(format!("{}", e)),
        };
        let jsoned: Value = match serde_json::from_str(&config_file) {
            serde_json::Result::Ok(j) => j,
            serde_json::Result::Err(e) => return Err(format!("{}", e)),
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
            Ok(f) => return Ok(sources_location),
            Err(e) => return Err(format!("{}", e)),
        }
    }

    Ok(sources_location.to_string())
}

fn main() -> Result<(), String> {
    let mut sources = init()?;

    let files_path_in_server = "/etc/yarpm/";

    let matches = App::new("yarpm")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Yet another rusty package manager")
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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        match search(
            &mut sources,
            matches.value_of("package").unwrap(),
            files_path_in_server,
        )? {
            Status::Found(s) => println!(
                "Found a package matching `{}` at {}",
                matches.value_of("package").unwrap(),
                s
            ),
            Status::NotFound => println!("Package not found"),
        }
    }

    Ok(())
}

enum Status {
    Found(String),
    NotFound,
}
fn search(sources: &String, package: &str, files_paths: &str) -> Result<Status, String> {
    let raw_sources = match fs::read_to_string(sources) {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };
    let sources_content = raw_sources.split("\n").collect::<Vec<&str>>();

    for source in sources_content {
        let full_path = &format!("{}{}{}.tar.gz", source, files_paths, package);
        let resp = match reqwest::blocking::get(full_path) {
            Ok(r) => r,
            Err(e) => return Err(format!("{}", e)),
        };

        if resp.status().is_success() {
            return Ok(Status::Found(full_path.to_string()));
        } else if let StatusCode::NOT_FOUND = resp.status() {
            continue;
        }
    }

    Ok(Status::NotFound)
}
