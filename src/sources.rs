use std::io::prelude::*;
use std::{fs, fs::File};

pub fn remove(source: &str, sources_path: &str) -> Result<(), String> {
    let content = match fs::read_to_string(sources_path) {
        Ok(c) => c,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    let mut towrite = String::new();

    for line in content.replace("\n", "").split("\n").collect::<Vec<&str>>() {
        if !line.contains(source) {
            towrite.push_str(line);
            towrite.push('\n');
        }
    }

    let mut file = match File::create(sources_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    match file.write_all(towrite.trim().as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }

    Ok(())
}

pub fn add(source: &str, sources_path: &str) -> Result<(), String> {
    let mut file_content = match fs::read_to_string(sources_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };
    file_content.push_str(source);
    file_content.push('\n');
    let mut file = match File::create(sources_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    };

    match file.write_all(file_content.as_bytes()) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(format!("{} - {}", line!(), e)),
    }
}
