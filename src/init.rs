use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::*;

pub fn init() -> anyhow::Result<Vec<Package>> {
  let sources_path = &format!(
    "{}/.werb.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  );

  let distant_path = "https://github.com/Wafelack/werb/raw/rewrite/packages.json";
  let binaries_path = &format!("{}/.werb_bin", dirs::home_dir().unwrap().to_str().unwrap());
  let path_export = &format!("export PATH=\"{}:$PATH\"", binaries_path);
  let bashrc_path = &format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap());

  if Path::new(sources_path).exists() {
     fs::remove_file(sources_path)?;
  }

  let bytes =  reqwest::blocking::get(distant_path)?
  .bytes()?;

  let mut f =  fs::File::create(sources_path)?;

  f.write_all(&bytes)?;
  if !Path::new(binaries_path).exists() {
     fs::create_dir_all(binaries_path)?;
  }

  let bashrc_content = fs::read_to_string(bashrc_path)?;
  let mut bashrc = fs::OpenOptions::new().append(true).open(bashrc_path)?;

  let mut already_written = false;

  for line in bashrc_content.split("\n").collect::<Vec<&str>>() {
    if line == path_export {
      already_written = true;
      break;
    }
  }

  if !already_written {
    bashrc.write_all(&format!("\n{}", path_export).as_bytes())?;
  }
  get_packages(&format!(
    "{}/.werb.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  ))
}


