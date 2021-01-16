use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Package {
  pub name: String,
  pub description: String,
  pub authors: Vec<String>,
  pub version: String,
  pub source: String,
}

impl Package {
  pub fn new(name: &str, authors: Vec<&str>, version: &str, source: &str, description: &str) -> Self {
    Self {
      name: name.to_owned(),
      authors: authors
        .iter()
        .map(|s| format!("{}", s))
        .collect::<Vec<String>>(),
      version: version.to_owned(),
      source: source.to_owned(),
      description: description.to_owned(),
    }
  }
}

impl std::fmt::Display for Package {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "{} - {}", self.name, self.version)?;
    writeln!(f, "By {}", self.authors.join(", "))?;
    writeln!(f, "{}", self.description)?;

    Ok(())
  }
}

pub fn get_packages(sources_path: &str) -> Result<Vec<Package>, String> {
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