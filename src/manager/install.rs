use crate::{Result, packager::installer::install};
use std::fs;

use super::download_package;

pub fn inst(package: &str) -> Result<()> {

    let fname = download_package(package)?;

    install(&fname)?;
    fs::remove_file(&fname)?;

    Ok(())
}