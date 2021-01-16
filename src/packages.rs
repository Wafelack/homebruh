use std::io::Write;
use std::fs;

use crate::utils::*;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn search(name :&str) -> Result<Package, String> {
    let packages = get_packages(&format!(
        "{}/.yarpm.sources",
        dirs::home_dir().unwrap().to_str().unwrap()
      ))?;

    for package in packages {
        if package.name == name {
            return Ok(package);
        }
    }

    return Err(format!("No packages matching `{}` were found", name));
}
pub fn install(name :&str) -> Result<(), String> {
    let binaries_path = &format!("{}/.yarpm_bin", dirs::home_dir().unwrap().to_str().unwrap());
    let package = search(name)?;
    let fp = &format!("{}/{}.tar.gz", binaries_path, package.name);

    let status = match reqwest::blocking::get(&package.source) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    }.status();

    if !status.is_success() {
        return Err(format!("HTTP error occured: code {}", status.as_u16()));
    }

    let mut choice = String::new();
    println!("{}", &package);
    print!("Do you want to install this package ? [y/N] ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut choice).unwrap();

    if choice.trim().to_uppercase() != "Y" {
        println!("Aborting");
        return Ok(())
    }

    let bytes = match {
        match reqwest::blocking::get(&package.source) {
            Ok( r) => r,
            Err(e) => return Err(e.to_string())
        }.bytes()
    } {
        Ok(b) => b.to_vec(),
        Err(e) => return Err(e.to_string()),
    };

    let mut raw = match fs::File::create(fp) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    match raw.write_all(&bytes) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    };

    let tar_gz = match fs::File::open(fp) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    match fs::remove_file(fp) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    };
    match archive.unpack(binaries_path) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    Ok(())
}