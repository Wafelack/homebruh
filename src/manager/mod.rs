pub mod sync;
pub mod install;

use std::{fs, fs::{File}, path::Path, io::Write};
use toml::Value;
use sha2::{Sha256, Digest};
use crate::{Result, Error};

fn download_package(package: &str) -> Result<String> {
    let packages_path = "/etc/homebruh/packages";

    let pkg = &format!("{}/{}.toml", packages_path, package);
    if !Path::new(pkg).exists() {
        return Err(
            Error::OtherError(format!("target not found: {}", package))
        )
    }
    let _pkg = toml::from_str::<Value>(&fs::read_to_string(pkg)?)?;
    let package_content = _pkg.as_table().unwrap();
    
    if !package_content.contains_key("link") || !package_content.contains_key("sha256") {
        return Err(
            Error::OtherError("Invalid package manifest.".to_owned())
        )
    }

    let sha256 = package_content["sha256"].as_str().unwrap();
    let link = package_content["link"].as_str().unwrap();
    let bytes = reqwest::blocking::get(link)?.bytes()?.to_vec();

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let fhash = format!("{:x}", hasher.finalize());

    if &fhash != sha256 {
        return Err(
            Error::OtherError(
                format!("Invalid sha256 hash.\nExpected: {}\nFound: {}", sha256, fhash)
            )
        )
    }

    let fname = format!("{}.bpkg", package);

    let mut pkg_file = File::create(&fname)?;
    pkg_file.write_all(&bytes)?;

    Ok(fname)
}