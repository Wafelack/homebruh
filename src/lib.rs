mod map;
use map::*;

#[derive(Clone)]
pub enum Object {
    Void,
    Player(Player),
}
#[derive(Clone)]
pub struct Player {
    name: String,
    hp: u32,
    repr: char,
}

impl Object {
    pub fn display(&self) {
        match self {
            Object::Void => print!(" "),
            Object::Player(player) => print!("{}", player.repr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let map = Map::new(10, 20);
        map.display();
    }
    #[test]
    fn add_obj() {
        let mut map = Map::new(10,20);
        assert!(map.add_object(Object::Player(Player {name: String::from("Test"), hp: 32, repr: '*'}), 5, 5).is_ok());
        map.display();
    }

    #[test]
    fn mov_obj() {
        let mut map = Map::new(10,20);
        assert!(map.add_object(Object::Player(Player {name: String::from("Test"), hp: 32, repr: '*'}), 5, 5).is_ok());
        assert!(map.move_object((5, 5), (0, 5)).is_ok());
        map.display();
    }
}



