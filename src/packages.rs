use crate::utils::*;

pub fn search(name :&str) -> Result<Package, String> {
    let packages = get_packages()?;

    for package in packages {
        if package.name == name {
            return Ok(package);
        }
    }

    return Err(format!("No packages matching `{}` were found", name));
}