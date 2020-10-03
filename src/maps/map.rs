use std::io::{self, Write};
use num_traits::sign::Unsigned;

pub struct GameMap {
    width: u8,
    height: u8,
    delimiter: char,
}

impl GameMap{
    pub fn new(width: u8, height: u8, delimiter: char) -> GameMap {
        GameMap{width, height, delimiter }
    }
    pub fn setsize(&mut self, width: u8, height: u8) {
        self.width = width;
        self.height = height;
    }
    pub fn setdelimiter(&mut self, delimiter: char) {
        self.delimiter = delimiter;
    }
    pub fn create(&self) {
        print!("\x1B[2J\x1B[1;1H");

        // Top lane
        for i in 0..(self.width + 1) {
            print!("{}", self.delimiter);
        }
        print!("\n");
        io::stdout().flush().unwrap();

        // Borders
        for i in 0..(&self.height+1) {
            print!("{}", self.delimiter);
            for j  in 0..(&self.width - 1) {
                print!(" ");
            }
            print!("{}", self.delimiter);
            print!("\n");
            io::stdout().flush().unwrap();

        }

        // Bottom lane
        for i in 0..(self.width + 1) {
            print!("{}", self.delimiter);
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}