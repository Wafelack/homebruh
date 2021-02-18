mod manager;
mod packager;

use manager::{
    install::{inst, uninst},
    sync::sync,
};

use packager::{builder::build, installer::install, uninstaller::uninstall};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "help" => help(),
            "build" => build()?,
            "install" => match args.next() {
                Some(package) => match args.next() {
                    Some(inner_package) => {
                        if package == "-i" {
                            install(inner_package)?;
                        }
                    }
                    None => inst(&package)?,
                },
                None => println!("Usage: {} install [-i] <package>.", env!("CARGO_PKG_NAME")),
            },
            "uninstall" => match args.next() {
                Some(package) => match args.next() {
                    Some(inner_package) => {
                        if package == "-i" {
                            uninstall(inner_package)?;
                        }
                    }
                    None => uninst(&package)?,
                },
                None => println!("Usage: {} uninstall [-i] <package>", env!("CARGO_PKG_NAME")),
            },
            "sync" => sync()?,
            _ => help(),
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
    println!("\tbuild                     \tBuilds the package reffering to `bruh.toml`.");
    println!("\tinstall -i $package_file  \tInstalls the specified package file.");
    println!("\tuninstall -i $package_file\tUninstalls the specified package file.");
    println!();
    println!("\tsync                      \tSynchronizes community database.");
    println!("\tinstall $package_name     \tInstalls the specified pacakge from the sources.");
    println!("\tuninstall $package_name   \tUninstalls the specified package.");

    println!();
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    TomlError(TomlError),
    OtherError(String),
    RequestError(reqwest::Error),
}

#[derive(Debug)]
pub enum TomlError {
    DeError(toml::de::Error),
    SerError(toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
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
