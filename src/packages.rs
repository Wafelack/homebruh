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
pub fn install(name :&str, confirm: bool, write: bool) -> anyhow::Result<()> {
    let binaries_path = &format!("{}/.werb_bin", dirs::home_dir().unwrap().to_str().unwrap());
    let package = search(name)?;
    let fp = &format!("{}/{}.tar.gz", binaries_path, package.name);

    let installed_path = &format!(
        "{}/.werb.installed",
        dirs::home_dir().unwrap().to_str().unwrap()
    );

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

    if write {
        let mut file = fs::OpenOptions::new().append(true).open(installed_path)?;
        if !is_installed(&package.name)? {
            file.write_all(format!("{}\r\n", package.name).as_bytes())?;
        } else {
            eprintln!("[ {} ] Package was already installed. Aborting", "ERR".red());
            return Ok(());
        }
    }

    if write {
        eprintln!("[ {} ] Successfully installed package {} version {}","OK".green(), package.name, package.version);
    }

    Ok(())
}

fn is_installed(name: &String) -> anyhow::Result<bool> {
    Ok( get_installed()?.contains(name))
}

fn get_installed() -> anyhow::Result<Vec<String>> {
    let installed_path = &format!(
        "{}/.werb.installed",
        dirs::home_dir().unwrap().to_str().unwrap()
    );
    Ok(fs::read_to_string(installed_path)?.trim().split(' ').map(|s| s.replace("\r", "")).filter(|s| s != "").collect::<Vec<String>>())
}

pub fn upgrade() -> anyhow::Result<()> {

    let packages = get_installed()?;

    eprintln!("[ {} ] Reading installed packages list", "OK".green());
    if packages.len() == 0 {
        eprintln!("[ {} ] No installed packages", "ERR".red());
        return Ok(());
    }

    for name in &packages {
        install(name.trim(), false, false)?;
    }

    eprintln!("[ {} ] Successfully upgraded {} {}.", "OK".green(), packages.len(),
        if packages.len() > 1 {
            "packages"
        } else {
            "package"
        }
    );

    Ok(())

}


pub fn purge(name :&String, confirm: bool) -> anyhow::Result<()> {
    let binaries_path = &format!("{}/.werb_bin", dirs::home_dir().unwrap().to_str().unwrap());
    let package = search(name)?;
    let fp = &format!("{}/{}.tar.gz", binaries_path, package.name);
    if !is_installed(name)? {
        eprintln!("[ {} ] Package `{}` isn't installed", "ERR".red(), name);
        return Ok(());
    }

    let status =  reqwest::blocking::get(&package.source)?.status();

    if !status.is_success() {
        return Err(anyhow::anyhow!(format!("HTTP error occured: code {}", status.as_u16())));
    }
    eprintln!("[ {} ] Found a package matching `{}`", "OK".green(), name);

    if confirm {
        let mut choice = String::new();
        println!("{}", &package);
        print!("Do you want to purge this package ? [y/N] ");
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

    let tar_gz = fs::File::open(fp)?;

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    fs::remove_file(fp)?;

    eprintln!("[ {} ] Getting decompressed archive", "OK".green());

    eprintln!("[ {} ] Reading archive", "OK".green());

    let entries = archive.entries()?;
    let mut i = 1;
    let mut dirs = Vec::new();

    use std::path::Path;

   for file in entries {
       let file = file.unwrap();
       let path = file.header().path().unwrap();
       let name = format!("{}/{}", binaries_path, path.to_str().unwrap());
       eprint!("\r       Removing package files ................. {} out of {}", i,"?" );

        if Path::new(&name).is_dir() {
            dirs.push(name);
            continue;
        }


       fs::remove_file(name)?;
       i += 1;

   }

    for dir in &dirs {
        fs::remove_dir_all(dir)?;
    }

    let installed_path = &format!(
        "{}/.werb.installed",
        dirs::home_dir().unwrap().to_str().unwrap()
    );

    let installed_packages = get_installed()?;
    let mut towrite = String::new();
    for package in installed_packages {
        if package.trim() != name {
            towrite.push_str(&format!("{}\r\n", package));
        }
    }

    let mut file = fs::File::create(installed_path)?;
    file.write_all(towrite.as_bytes())?;

    eprintln!("\rRemoving package files ................. {} out of {}", i,i );

    eprintln!("[ {} ] Successfully purged package {}", "OK".green(), package.name);

    Ok(())
}