use std::{ffi::OsStr, fmt::Display, fs, fs::File, path::Path, process::Command};

use crate::Error;

use flate2::read::GzDecoder;
use tar::Archive;
use toml::{value::Map, Value};

pub fn install_local<T>(input: T) -> crate::Result<()>
where
    T: AsRef<Path> + AsRef<OsStr> + Display,
{
    if !Path::new(&input).exists() {
        return Err(Error::Other(format!("Cannot find file `{}`.", &input)));
    }

    let dest_folder = &input.to_string().replace(".bpkg", "");

    println!("\x1b[0;32mDecompressing\x1b[0m `{}`...", &input);

    let tar_gz = File::open(&input)?;
    Archive::new(GzDecoder::new(tar_gz)).unpack(dest_folder)?;

    println!("\x1b[0;32mSucessfully\x1b[0m decompressed `{}`.", &input);

    let manifest_path = &format!("{}/bruh.toml", dest_folder);

    let map: Map<String, Value> = {
        if let Ok(t) = fs::read_to_string(manifest_path) {
            println!("\x1b[0;32mReading\x1b[0m manifest information...");
            toml::from_str(&t)?
        } else {
            fs::remove_dir_all(dest_folder)?;
            return Err(Error::Other(
                "Cannot find `bruh.toml` in the package.".to_owned(),
            ));
        }
    };

    if !map.contains_key("name") || !map.contains_key("version") || !map.contains_key("files") {
        fs::remove_dir_all(dest_folder)?;
        return Err(Error::Other(
            "One or more keys are missing from manifest.".to_string(),
        ));
    }

    if let Some(value) = map.get("startup_script") {
        println!("\x1b[0;32mExecuting\x1b[0m startup script...");

        let script = format!("{}/{}", dest_folder, value.as_str().unwrap());

        if !Path::new(&script).exists() {
            fs::remove_dir_all(dest_folder)?;
            return Err(Error::Other(format!(
                "Cannot find `{}` in the package.",
                value.as_str().unwrap()
            )));
        }

        let status = Command::new(&script).status()?;

        if !status.success() {
            fs::remove_dir_all(dest_folder)?;
            return Err(Error::Other(format!(
                "Startup script exited with an error code: {}.",
                status.code().unwrap_or(-1)
            )));
        }
    }

    let name = map["name"].as_str().unwrap();
    let version = map["version"].as_str().unwrap();
    let fs_path = &format!("{}/{}", dest_folder, map["files"].as_str().unwrap());

    println!("\x1b[0;32mCopying\x1b[0m package files...");

    let dir = super::see_dir(fs_path)?;

    for (i, file) in dir.iter().enumerate() {
        let dest = &format!("/{}", &file.replace(fs_path, ""));

        if !Path::new(dest).exists() {
            File::create(dest)?;
        }

        fs::copy(&file, dest)?;

        print!("\r{}-{} [", name, version);
        for _ in 0..((i / dir.len()) * 20) {
            print!("#");
        }
        for _ in 0..((dir.len() - i) / dir.len() * 20) {
            print!("-");
        }
        print!("] {}/{}", i, &dir.len());
    }
    println!();

    if let Some(value) = map.get("cleanup_script") {
        println!("\x1b[0;32mExecuting\x1b[0m cleanup script...");
        let script = format!("{}/{}", dest_folder, value.as_str().unwrap());

        if !Path::new(&script).exists() {
            fs::remove_dir_all(dest_folder)?;
            return Err(Error::Other(format!(
                "Cannot find `{}` in the package.",
                value.as_str().unwrap()
            )));
        }

        let status = Command::new(&script).status()?;

        if !status.success() {
            fs::remove_dir_all(dest_folder)?;
            return Err(Error::Other(format!(
                "Startup script exited with an error code: {}.",
                status.code().unwrap_or(-1)
            )));
        }
    }

    println!("\x1b[0;32mCleaning\x1b[0m packages files");
    fs::remove_dir_all(dest_folder)?;
    println!(
        "\x1b[0;32mSucessfully\x1b[0m installed {} v{}.",
        name, version
    );

    Ok(())
}
