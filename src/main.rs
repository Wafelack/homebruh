use std::todo;

use packager::{builder::build, unbuilder::unbuild};

mod packager;

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() > 0 {
        if args.contains(&"--help".to_owned()) {
            help();
        } else if args.contains(&"--version".to_owned()) {
            println!("{} {}",env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        } else {
            match args[0].as_str() {
                "build" => build()?,
                "install" => if args.len() == 2 {
                    todo!()
                } else if args.len() == 3 && &args[1] == "-i" {
                    unbuild(&args[2])?;
                } else {
                    println!("Usage: {} unpack [-i] <package>.", env!("CARGO_PKG_NAME"));
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn help() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));

    // Usage
    println!("\nUSAGE:");
    println!("\n{} <COMMAND> [OPTIONS]", env!("CARGO_PKG_NAME"));

    // Flags
    println!("\nFLAGS:");
    println!("\t--help   \tDisplays this message.");
    println!("\t--version\tDisplays version information.");

    // Commands
    println!("\nCOMMANDS:");
    println!("\tbuild                   \tBuilds the package reffering to `bruh.toml`.");
    println!("\tinstall -i $package_file\tInstalls the specified package file.");
    println!("\tinstall $package        \tInstalls the specified package from a repository.");

    println!();
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    TomlError(TomlError),
    OtherError(String),
}
#[derive(Debug)]
pub enum TomlError {
    DeError(toml::de::Error),
    SerError(toml::ser::Error)
}

pub type Result<T> = std::result::Result<T, Error>;


impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self::TomlError(TomlError::SerError(e))
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::TomlError(TomlError::DeError(e))
    }
}