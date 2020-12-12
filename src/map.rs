use crate::Object;
use std::error::Error;

pub struct Map {
  content: Vec<Vec<Object>>,
  height: usize,
  width: usize,
}

impl Map {
  pub fn new(height: usize, width: usize) -> Self {
    Self { content: vec![vec![Object::Void; width]; height], height, width }
  }
  pub fn add_object<'a>(&mut self, object: Object, y: usize, x: usize) -> Result<(), &'a str> {
    if !(x > self.width || y > self.height) {
      self.content[x][y] = object;
      return Ok(());
    } else {
      return Err("Invalid height or value");
    }
  }
  pub fn move_object<'a>(&mut self, (from_x, from_y): (usize, usize), (to_x, to_y): (usize, usize)) -> Result<(), &'a str> {
    if !(from_x > self.width || from_y > self.height || to_x > self.width || to_y > self.height) {
      self.content[to_x][to_y] = self.content[from_x][from_y].clone();
      self.content[from_x][from_y] = Object::Void;
      return Ok(());
    } else {
      return Err("Invalid height or value");
    }
  }
  pub fn display(&self) {
    print!(" ");
    for _ in 0..self.width {
      print!("_");
    }
    println!(" ");
    for y in 0..self.height {
      print!("|");
      for x in 0..self.width {
        
          self.content[y][x].display();
      }
        print!("|");
        println!();
    }

    print!(" ");
    for _ in 0..self.width {
      print!("¯");
    }
    println!(" ");
  }
}