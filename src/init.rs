use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::*;

pub fn init() -> Result<Vec<Package>, String> {
  let sources_path = &format!(
    "{}/.yarpm.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  );

  let distant_path = "https://github.com/Wafelack/yarpm/raw/rewrite/packages.json";
  let binaries_path = &format!("{}/.yarpm_bin", dirs::home_dir().unwrap().to_str().unwrap());
  let path_export = &format!("export PATH=\"{}:$PATH\"", binaries_path);
  let bashrc_path = &format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap());

  if Path::new(sources_path).exists() {
    match fs::remove_file(sources_path) {
      Ok(()) => {}
      Err(e) => return Err(e.to_string()),
    }
  }

  let r_body = match reqwest::blocking::get(distant_path) {
    Ok(r) => r,
    Err(e) => return Err(e.to_string()),
  }
  .bytes();

  let bytes = match r_body {
    Ok(b) => b.to_vec(),
    Err(e) => return Err(e.to_string()),
  };

  let mut f = match fs::File::create(sources_path) {
    Ok(f) => f,
    Err(e) => return Err(e.to_string()),
  };

  match f.write_all(&bytes) {
    Ok(()) => {}
    Err(e) => return Err(e.to_string()),
  }
  if !Path::new(binaries_path).exists() {
    match fs::create_dir_all(binaries_path) {
      Ok(()) => {}
      Err(e) => return Err(e.to_string()),
    }
  }

  let bashrc_content = match fs::read_to_string(bashrc_path) {
    Ok(s) => s,
    Err(e) => return Err(e.to_string()),
  };
  let mut bashrc = match fs::OpenOptions::new().append(true).open(bashrc_path) {
    Ok(f) => f,
    Err(e) => return Err(e.to_string()),
  };

  let mut already_written = false;

  for line in bashrc_content.split("\n").collect::<Vec<&str>>() {
    if line == path_export {
      already_written = true;
      break;
    }
  }

  if !already_written {
    match bashrc.write_all(&format!("\n{}", path_export).as_bytes()) {
      Ok(()) => {}
      Err(e) => return Err(e.to_string()),
    }
  }
  get_packages()
}


