# wcli_game_engine
Simple game engine to make simple CLI games

## Functions

### Map

```rs
pub fn new(width: u8, height: u8, delimiter: char) -> GameMap
```
```rs
pub fn setsize(&mut self, width: u8, height: u8)
```
```rs
pub fn setdelimiter(&mut self, delimiter: char)
```
```rs
pub fn create(&self)
```
```rs
pub fn drawentities(&self, entities: &Vec<Entity>)
```

### Entity

```rs
pub fn new(x: u8, y: u8, isplayer: bool, symbol: char, canmove: bool) -> Entity
```
```rs
pub fn newdefault(symbol: char) -> Entity
```
```rs
pub fn get_pos(&self) -> (u8, u8)
```
```rs
pub fn move_to(&mut self, x: u8, y: u8)
```
```rs
pub fn symbol(&self) -> char
```

## Documentation

Create a new empty map and display it to the screenx:

```rs
let map = GameMap::new(50,8,"#");
map.create();
```

Resize it and change the border:
```rs
let mut map = GameMap::new(50, 8, "#");
map.create();
map.setsize(40, 8);
map.create();
map.setdelimiter('*');
map.create();
```

Draw some entities :
```rs
let mut map = GameMap::new(50,10,'#');
let entities: Vec<Entity> = vec![Entity::new(10,15,false, '8', false)] 
map.drawentities(entities); // Won't draw anything cause 15 > 10
```

```rs
let mut map = GameMap::new(50,10,'#');
let entities: Vec<Entity> = vec![Entity::new(10,8,false, '8', false), Entity::newdefault('0')] 
map.drawentities(entities); // Will draw a 8 at (10;8) and a 0 at (1;1)
```

Move an entity after drawing it :

```rs
let mut map = GameMap::new(50,10,'#');
let mut entities: Vec<Entity> = vec![Entity::newdefault('0')] 
map.drawentities(&entities); // Will draw a 0 at (1;1)
entities[0].move_to(5,2);
map.drawentities(&entities); // Will draw a 0 at (5;2)
```
