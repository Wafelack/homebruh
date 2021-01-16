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

pub fn get_packages(sources_path: &str) -> anyhow::Result<Vec<Package>> {
  let packages_content = fs::read_to_string(sources_path)?;

  let packages: Vec<Package> = serde_json::from_str(&packages_content)?;
  Ok(packages)
  
}

  pub fn pretty_bytes(bytes: usize) -> String {
    if bytes > 1000000000 {
      format!("{} GB", bytes as f32 / 1000000000.)
    } else if bytes > 1000000 {
      format!("{} MB", bytes as f32 / 1000000.)
    } else if bytes > 1000 {
      format!("{} kB", bytes as f32 / 1000.)
    } else {
      format!("{} B", bytes)
    }
  }