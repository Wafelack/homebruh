use std::fs;

use crate::{
    packager::{installer::install_local, uninstaller::uninstall_local},
    Result,
};

use super::download_package;

pub fn install_remote(package: &str) -> Result<()> {
    let fname = download_package(package)?;

    install_local(&fname)?;
    fs::remove_file(&fname)?;

    Ok(())
}

pub fn uninstall_remote(package: &str) -> Result<()> {
    let fname = download_package(package)?;

    uninstall_local(&fname)?;
    fs::remove_file(&fname)?;

    Ok(())
}
