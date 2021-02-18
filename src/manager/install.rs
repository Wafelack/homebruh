use std::fs;

use crate::{
    packager::{installer::install, uninstaller::uninstall},
    Result,
};

use super::download_package;

pub fn inst(package: &str) -> Result<()> {
    let fname = download_package(package)?;

    install(&fname)?;
    fs::remove_file(&fname)?;

    Ok(())
}

pub fn uninst(package: &str) -> Result<()> {
    let fname = download_package(package)?;

    uninstall(&fname)?;
    fs::remove_file(&fname)?;

    Ok(())
}
