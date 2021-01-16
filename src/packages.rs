use std::io::Write;
use std::fs;
use colored::*;

use crate::utils::*;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn search(name :&str) -> anyhow::Result<Package> {
    let packages = get_packages(&format!(
        "{}/.werb.sources",
        dirs::home_dir().unwrap().to_str().unwrap()
      ))?;

    for package in packages {
        if package.name == name {
            return Ok(package);
        }
    }

    Err(
        anyhow::anyhow!(format!("Cannot find package `{}`", name))
    )
}
pub fn install(name :&str, confirm: bool) -> anyhow::Result<()> {
    let binaries_path = &format!("{}/.werb_bin", dirs::home_dir().unwrap().to_str().unwrap());
    let package = search(name)?;
    let fp = &format!("{}/{}.tar.gz", binaries_path, package.name);

    let status =  reqwest::blocking::get(&package.source)?.status();

    if !status.is_success() {
        return Err(anyhow::anyhow!(format!("HTTP error occured: code {}", status.as_u16())));
    }
    eprintln!("[ {} ] Found a package matching `{}`", "OK".green(), name);

    if confirm {
        let mut choice = String::new();
        println!("{}", &package);
        print!("Do you want to install this package ? [y/N] ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut choice)?;
    
        if choice.trim().to_uppercase() != "Y" {
            println!("Aborting");
            return Ok(())
        }
    }

    let bytes = reqwest::blocking::get(&package.source)?.bytes()?.to_vec();

    eprintln!("[ {} ] Downloaded {} from {}", "OK".green(), pretty_bytes(bytes.len()), &package.source);

    let mut raw = fs::File::create(fp)?;
    raw.write_all(&bytes)?;

    eprintln!("[ {} ] Installed archive", "OK".green());

    let tar_gz = fs::File::open(fp)?;

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    fs::remove_file(fp)?;

    eprintln!("[ {} ] Decompressed archive", "OK".green());

    archive.unpack(binaries_path)?;

    eprintln!("[ {} ] Unpacked archive", "OK".green());

    eprintln!("Successfully installed package {} version {}", package.name, package.version);

    Ok(())
}