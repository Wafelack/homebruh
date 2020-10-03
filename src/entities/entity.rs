pub struct Entity {
    posx: u8,
    posy: u8,
    isplayer: bool,
    symbol: char,
    canmove: bool,
}

impl Entity {
    pub fn new(x: u8, y: u8, isplayer: bool, symbol: char, canmove: bool) -> Entity {
        Entity{posx: x, posy: y, isplayer, symbol, canmove}
    }
    pub fn newdefault(symbol: char) -> Entity {
        Entity{posx: 0, posy: 0, isplayer: false, symbol, canmove: false}
    }
    pub fn get_pos(&self) -> (u8, u8) {
        (self.posx, self.posy)
    }
    pub fn move_to(&mut self, x: u8, y: u8) {
        self.posx = x;
        self.posy = y;
    }
    pub fn symbol(&self) -> char {
        self.symbol
    }
}