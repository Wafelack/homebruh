use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Package {
  name: String,
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
