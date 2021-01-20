use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::*;

pub fn init() -> anyhow::Result<Vec<Package>> {
  let sources_path = &format!(
    "{}/.werb.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  );

  let installed_path = &format!(
    "{}/.werb.installed",
    dirs::home_dir().unwrap().to_str().unwrap()
  );

  let distant_path = "https://raw.githubusercontent.com/Wafelack/werb/dev/packages.json";
  let binaries_path = &format!("{}/.werb_bin", dirs::home_dir().unwrap().to_str().unwrap());

  if Path::new(sources_path).exists() {
     fs::remove_file(sources_path)?;
  }

  if !Path::new(installed_path).exists() {
    fs::File::create(installed_path)?;
  }

  let bytes =  reqwest::blocking::get(distant_path)?
  .bytes()?;

  let mut f =  fs::File::create(sources_path)?;

  f.write_all(&bytes)?;
  if !Path::new(binaries_path).exists() {
     fs::create_dir_all(binaries_path)?;
  }

  get_packages(&format!(
    "{}/.werb.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  ))
}


