use crate::{Result, Error, packager::installer::install};
use std::{env, fs, io::Write, path::Path};
use fs::{File, remove_file};
use toml::Value;
use sha2::{Digest, Sha256};

use super::download_package;

pub fn inst(package: &str) -> Result<()> {

    let fname = download_package(package)?;

    install(&fname)?;
    fs::remove_file(&fname)?;
    

    Ok(())
}