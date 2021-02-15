use std::{ffi::OsStr, fmt::Display, fs, path::Path, time::Instant};
use flate2::{Compression, write::GzEncoder};
use fs::File;
use tar::Builder;
use toml::Value;

use crate::{Error, Result};


/// Package file format:
///
/// name = "foo"
/// version = "0.1.0"
/// on_start = "startup.sh"
/// on_end = "cleanup.sh"
/// files = "foo.tar.gz"
pub fn build<T>(input: T) -> Result<()>
where T: AsRef<Path> + AsRef<OsStr> + Display {

    if !Path::new(&input).exists() {
        return Err(
            Error::OtherError(format!("Cannot find `{}` in this directory.", input))
        )
    }

    let start = Instant::now();

    let tomlized: Value = toml::from_str(&fs::read_to_string(&input)?)?;

    let map = tomlized.as_table().unwrap();

    if !map.contains_key("name") || !map.contains_key("version") || !map.contains_key("files") {
        return Err(
            Error::OtherError(format!("One or more keys are missing from `{}`.", &input))
        )
    }

    if !Path::new(map["files"].as_str().unwrap()).exists() {
        return Err(
            Error::OtherError(format!("Cannot find directory `{}`.", map["files"].as_str().unwrap()))
        )
    }

    println!("\x1b[0;32mPackaging\x1b[0m `{}` v{}...", map["name"].as_str().unwrap(), map["version"].as_str().unwrap());

    let file = File::create(&format!("{}-{}.tar.gz", map["name"].as_str().unwrap(), map["version"].as_str().unwrap()))?;

    println!("\x1b[0;32mCreating\x1b[0m archive...");


    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_path(input)?;
    println!("\x1b[0;32mPackaging\x1b[0m package manifest...");
    tar.append_dir_all(map["files"].as_str().unwrap(), map["files"].as_str().unwrap())?;
    println!("\x1b[0;32mPackaging\x1b[0m package files...");
    if map.contains_key("on_start") {
        tar.append_path(map["on_start"].as_str().unwrap())?;
        println!("\x1b[0;32mPackaging\x1b[0m on_start script...");
    }

    if map.contains_key("on_end") {
        tar.append_path(map["on_end"].as_str().unwrap())?;
        println!("\x1b[0;32mPackaging\x1b[0m on_end script...");
    }

    println!("\x1b[0;32mFinished\x1b[0m packaging `{}` v{} in {:.2}s.", map["name"].as_str().unwrap(), map["version"].as_str().unwrap(), start.elapsed().as_secs_f32());

    Ok(())
}