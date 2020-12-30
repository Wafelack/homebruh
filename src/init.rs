use std::io::prelude::*;
use std::path::Path;
use std::{fs, fs::File};

pub fn init() -> Result<(String, String, String), String> {
  let installed_list = "/etc/yarpm.installed";
  let default_sources = "/etc/yarpm.sources";
  let binaries_path = &format!("{}/.yarpm/bin", dirs::home_dir().unwrap().to_str().unwrap());
  let path_export = &format!("export PATH=\"{}:$PATH\"", binaries_path);
  let bashrc_path = &format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap());

  if !Path::new(installed_list).exists() {
    match File::create(installed_list) {
      Ok(_) => {}
      Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
  }

  let content = match fs::read_to_string(installed_list) {
    Ok(s) => s,
    Err(e) => return Err(format!("{} - {}", line!(), e)),
  };

  if content.is_empty() {
    let mut f = match File::create(installed_list) {
      Ok(f) => f,
      Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    match f.write_all(b"# This file is not intended for manual editing\n# Manual manipulation could alterate yarpm functionning\n\n") {
          Ok(()) => {},
          Err(e) => return Err(format!("{} - {}", line!(), e)),
      }
  }

  if !Path::new(binaries_path).exists() {
    match fs::create_dir_all(binaries_path) {
      Ok(()) => {}
      Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
  }
  let bashrc_content = match fs::read_to_string(bashrc_path) {
    Ok(s) => s,
    Err(e) => return Err(format!("{} - {}", line!(), e)),
  };
  let mut bashrc = match fs::OpenOptions::new().append(true).open(bashrc_path) {
    Ok(f) => f,
    Err(e) => return Err(format!("{} - {}", line!(), e)),
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
      Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
  }

  let sources_location = default_sources.to_string();

  if !Path::new(&sources_location).exists() {
    match File::create(&sources_location) {
      Ok(_) => {
        return Ok((
          sources_location,
          binaries_path.to_string(),
          installed_list.to_string(),
        ))
      }
      Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
  }

  Ok((
    sources_location.to_string(),
    binaries_path.to_string(),
    installed_list.to_string(),
  ))
}
