use std::{ffi::OsStr, fmt::Display, fs, path::Path, time::Instant};
use flate2::{Compression, write::GzEncoder, read::GzDecoder};
use fs::File;
use tar::Archive;
use toml::Value;

use crate::{Error, Result};


/// Package file format:
///
/// name = "foo"
/// version = "0.1.0"
/// on_start = "startup.sh"
/// on_end = "cleanup.sh"
/// files = "foo.tar.gz"
pub fn unbuild<T>(input: T) -> Result<()>
where T: AsRef<Path> + AsRef<OsStr> + Display + ToString {

    if !Path::new(&input).exists() {
        return Err(
            Error::OtherError(format!("Cannot find file `{}`.", &input))
        )
    }

    let tar_gz = File::open(&input)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&input.to_string().replace(".tar.gz", ""))?;

    Ok(())
}