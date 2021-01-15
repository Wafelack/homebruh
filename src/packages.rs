use crate::utils::*;

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