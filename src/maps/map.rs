use std::io::{self, Write};
use crate::entities::entity::*;

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
        io::stdout().flush().unwrap();

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
    pub fn drawentities(&self, entities: &Vec<Entity>) {

        // Security check
        for entity in entities {
            if entity.get_pos().0 >= self.width || entity.get_pos().1 >= self.height || entity.get_pos().0 <= 0 || entity.get_pos().1 <= 0 {
                return
            }
        }

        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        // Top lane
        for i in 0..(self.width + 1) {
            print!("{}", self.delimiter);
        }
        print!("\n");
        io::stdout().flush().unwrap();
        let mut nonefound = true;

        // Borders
        for i in 0..(&self.height+1) {
            print!("{}", self.delimiter);
            for j  in 0..(&self.width - 1) {
                for entity in entities {
                    if entity.get_pos() == (j, i) {
                        print!("{}", entity.symbol());
                        nonefound = false;
                    }
                }
                if nonefound {
                    print!(" ");
                }
                nonefound = true;
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