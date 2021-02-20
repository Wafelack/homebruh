use std::{fs::File, io::Write, path::Path};

pub mod install;
pub mod sync;

use crate::Error;

use sha2::{Digest, Sha256};
use toml::Value;

/// Attempts to download `package`, and will return the install
/// path if the download is successful
fn download_package(package: &str) -> crate::Result<String> {
    let packages_path = "/etc/homebruh/packages";

    let pkg = format!("{}/{}.toml", packages_path, package);

    if !Path::new(&pkg).exists() {
        return Err(Error::Other(format!("target not found: {}", package)));
    }

    let pkg = toml::from_str::<Value>(&std::fs::read_to_string(pkg)?)?;
    let package_content = pkg.as_table().unwrap();

    if !package_content.contains_key("link") || !package_content.contains_key("sha256") {
        return Err(Error::Other("Invalid package manifest.".to_string()));
    }

    let sha256 = package_content["sha256"].as_str().unwrap();
    let link = package_content["link"].as_str().unwrap();
    let bytes = reqwest::blocking::get(link)?.bytes()?;

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let fhash = format!("{:x}", hasher.finalize());

    if fhash != sha256 {
        return Err(Error::Other(format!(
            "Invalid sha256 hash.\nExpected: {}\nFound: {}",
            sha256, fhash
        )));
    }

    let fname = format!("{}.bpkg", package);

    let mut pkg_file = File::create(&fname)?;
    pkg_file.write_all(&bytes)?;

    Ok(fname)
}
