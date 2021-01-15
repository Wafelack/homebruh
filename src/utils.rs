use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Package {
  pub name: String,
  description: String,
  authors: Vec<String>,
  version: String,
  source: String,
}

impl Package {
  pub fn new(name: &str, authors: Vec<&str>, version: &str, source: &str) -> Self {
    Self {
      name: name.to_owned(),
      authors: authors
        .iter()
        .map(|s| format!("{}", s))
        .collect::<Vec<String>>(),
      version: version.to_owned(),
      source: source.to_owned(),
    }
  }
}

impl std::fmt::Display for Package {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "")
    Ok(())
  }
}

pub fn get_packages() -> Result<Vec<Package>, String> {
  let sources_path = &format!(
    "{}/.yarpm.sources",
    dirs::home_dir().unwrap().to_str().unwrap()
  );
  let packages_content = match fs::read_to_string(sources_path) {
    Ok(s) => s.to_owned(),
    Err(e) => return Err(e.to_string()),
  };

  let packages: Vec<Package> = match serde_json::from_str(&packages_content) {
    Ok(v) => v,
    Err(e) => return Err(e.to_string()),
  };
  Ok(packages)
}