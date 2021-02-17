use std::{env, fs, io::Write, path::Path, str};

use fs::File;

use crate::{Result, Error};

pub fn update() -> Result<()> {
    let community_sources_link = "https://raw.githubusercontent.com/Wafelack/homebruh/dev/community/packages.list";
    let packages_path = match env::var("HOME") {
        Ok(s) => format!("{}/.homebruh/packages", s),
        Err(_) => return Err(
            Error::OtherError("Cannot find $HOME variable.".to_owned())
        ) 
    };

    if !Path::new(&packages_path).exists() {
        println!("\x1b[0;32mCreating\x1b[0m local package repository.");
        fs::create_dir_all(&packages_path)?;
    }

    let content = reqwest::blocking::get(community_sources_link)?.bytes()?.to_vec();
    println!("\x1b[0;32mReading\x1b[0m package database.");
    let lines = str::from_utf8(&content).unwrap().lines();
    let len = lines.clone().collect::<Vec<_>>().len();

    println!("\x1b[0;32mDownloading\x1b[0m packages manifests.");
    for (i, line) in lines.enumerate() {
        let link = format!("https://raw.githubusercontent.com/Wafelack/homebruh/dev/community/{}.toml", line);
    
        let fcontent = reqwest::blocking::get(&link)?.bytes()?.to_vec();


        let path = format!("{}/{}.toml", &packages_path, line);

        let mut f = File::create(path)?;
        f.write_all(&fcontent)?;

        print!("[");
        for _ in 0..(i/len*50) {
            print!("#");
        }
        for _ in 0..((len-i)/len*50) {
            print!("-");
        }
        print!("] {}/{}", i, len);
    }

    println!();
    println!("\x1b[0;32mSucessfully\x1b[0m synchronized package database.");

    Ok(())
}