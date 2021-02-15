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
                "build" => if args.len() == 2 {
                    build(&args[1])?;
                } else {
                    println!("Usage: {} build <manifest>.", env!("CARGO_PKG_NAME"));
                },
                "unpack" => if args.len() == 2 {
                    unbuild(&args[1])?;
                } else {
                    println!("Usage: {} unpack <package>.", env!("CARGO_PKG_NAME"));
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
    println!("\tbuild $manifest\tBuilds the specified package manifest.");

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