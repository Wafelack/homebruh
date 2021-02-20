use crate::{Error, Result};
use flate2::read::GzDecoder;
use std::{ffi::OsStr, fmt::Display, fs, fs::File, path::Path};
use tar::Archive;
use toml::Value;

pub fn uninstall_local<T>(input: T) -> Result<()>
where
    T: AsRef<Path> + AsRef<OsStr> + Display + ToString,
{
    if !Path::new(&input).exists() {
        return Err(Error::Other(format!("Cannot find file `{}`.", &input)));
    }

    let dest_folder = &input.to_string().replace(".bpkg", "");

    println!("\x1b[0;32mDecompressing\x1b[0m `{}`...", &input);

    let tar_gz = File::open(&input)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dest_folder)?;
    println!("\x1b[0;32mSucessfully\x1b[0m decompressed `{}`.", &input);

    let manifest_path = &format!("{}/bruh.toml", dest_folder);

    if !Path::new(manifest_path).exists() {
        fs::remove_dir_all(dest_folder)?;
        return Err(Error::Other(
            "Cannot find `bruh.toml` in the package.".to_owned(),
        ));
    }

    println!("\x1b[0;32mReading\x1b[0m manifest information...");

    let manifest: Value = toml::from_str(&fs::read_to_string(manifest_path)?)?;
    let map = manifest.as_table().unwrap();

    if !map.contains_key("name") || !map.contains_key("version") || !map.contains_key("files") {
        fs::remove_dir_all(dest_folder)?;
        return Err(Error::Other(
            "One or more keys are missing from manifest.".to_string(),
        ));
    }

    let name = map["name"].as_str().unwrap();
    let version = map["version"].as_str().unwrap();
    let fs_path = &format!("{}/{}", dest_folder, map["files"].as_str().unwrap());

    println!("\x1b[0;32mDeleting\x1b[0m package files...");

    let dir = super::see_dir(fs_path)?;

    for (i, file) in dir.iter().enumerate() {
        let dest = &format!("/{}", &file.replace(fs_path, ""));

        if Path::new(dest).exists() {
            fs::remove_file(dest)?;
        }

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
    println!(
        "\x1b[0;32mSucessfully\x1b[0m uninstalled {} v{}.",
        name, version
    );

    Ok(())
}
