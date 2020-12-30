mod init;
mod packages;
mod sources;

use clap::{App, Arg, SubCommand};
use init::init;
use packages::*;
use sources::*;

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
