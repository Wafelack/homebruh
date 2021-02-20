pub mod builder;
pub mod installer;
pub mod uninstaller;

use std::{ffi::OsStr, fs, path::Path};

/// Iterates over dir, returning a list of file names
fn see_dir<T>(dir: T) -> crate::Result<Vec<String>>
where
    T: AsRef<Path> + AsRef<OsStr>,
{
    let mut toret = Vec::new();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;

        if entry.path().is_dir() {
            toret.extend(see_dir(entry.path())?);
        } else {
            toret.push(entry.path().to_str().unwrap().to_string())
        }
    }

    Ok(toret)
}
