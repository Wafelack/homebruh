mod init;
mod utils;
mod packages;

use clap::{Arg,App, SubCommand};
use packages::{install, search, purge};

fn main() -> anyhow::Result<()> {
    let matches = App::new("werb")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .about("Yet another rusty package manager")
                    .subcommand(SubCommand::with_name("search")
                            .arg(Arg::with_name("package")
                                    .required(true)
                                    .takes_value(true)
                                    .index(1))
                    )
        .subcommand(SubCommand::with_name("purge")
            .arg(Arg::with_name("package")
                .index(1)
                .required(true)
                .takes_value(true)))
                    .subcommand(SubCommand::with_name("install")
                        .arg(Arg::with_name("package")
                            .index(1)
                            .required(true)
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("force")
                            .short("f")
                            .long("force")
                            .takes_value(false)))
                    .get_matches();
    init::init()?;

    if let Some(matches) = matches.subcommand_matches("search") {
        println!("{}", search(matches.value_of("package").unwrap())?);
    } else if let Some(matches) = matches.subcommand_matches("install") {
        install(matches.value_of("package").unwrap(), !matches.is_present("force"))?;
    } else if let Some(matches) = matches.subcommand_matches("purge") {
        purge(matches.value_of("package").unwrap())?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::*;
    use super::*;


    #[test]
    fn installation() ->  anyhow::Result<()> {
        init::init()?;
        packages::install("nixt", false)?;
        Ok(())
    }
    #[test]
    fn search_package() -> anyhow::Result<()> {
        let packages = get_packages("packages.json")?;

        let mut has_to_be_here: Option<Package> = None;

        for package in packages {
            if package.name == "nixt" {
                has_to_be_here = Some(package);
            }
        }

        assert!(has_to_be_here.is_some());
        Ok(())
    }

    #[test]
    fn package_serialization() {
        let to_ser = Package::new(
            "wng",
            vec!["Wafelack <wafelack@protonmail.com>"],
            "3.5.0",
            "N/A",
            "Wanager is a package and projects manager for C and C++"
        );
        let serialized = serde_json::to_string(&to_ser).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"wng","description":"Wanager is a package and projects manager for C and C++","authors":["Wafelack <wafelack@protonmail.com>"],"version":"3.5.0","source":"N/A"}"#
        );
    }

    #[test]
    fn package_deserialization() {
        let to_deser = r#"{"name":"wng", "description":"Wanager is a package and projects manager for C and C++", "authors":["Wafelack <wafelack@protonmail.com>"],"version":"3.5.0","source":"N/A"}"#;
        let deserialized: Package = serde_json::from_str(&to_deser).unwrap();
        assert_eq!(
            deserialized,
            Package::new(
                "wng",
                vec!["Wafelack <wafelack@protonmail.com>"],
                "3.5.0",
                "N/A",
                "Wanager is a package and projects manager for C and C++"
            )
        );
    }
}
