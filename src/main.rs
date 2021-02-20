mod manager;
mod packager;

use manager::{
    install::{install_remote, uninstall_remote},
    sync::sync,
};

use packager::{builder::build, installer::install_local, uninstaller::uninstall_local};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "build" => build()?,
            "install" => match args.next() {
                Some(package) => match args.next() {
                    Some(inner_package) => {
                        if package == "-i" {
                            install_local(inner_package)?;
                        }
                    }
                    None => install_remote(&package)?,
                },
                None => println!("Usage: {} install [-i] <package>.", env!("CARGO_PKG_NAME")),
            },
            "uninstall" => match args.next() {
                Some(package) => match args.next() {
                    Some(inner_package) => {
                        if package == "-i" {
                            uninstall_local(inner_package)?;
                        }
                    }
                    None => uninstall_remote(&package)?,
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
    Io(std::io::Error),
    Toml(TomlError),
    Other(String),
    Request(reqwest::Error),
}

#[derive(Debug)]
pub enum TomlError {
    DeError(toml::de::Error),
    SerError(toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self::Toml(TomlError::SerError(e))
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(TomlError::DeError(e))
    }
}
