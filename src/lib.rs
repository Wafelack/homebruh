mod maps;
use maps::map::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptymap() {
        let map = GameMap::new(50,8,'#');
        map.create();
    }
    #[test]
    fn resizing() {
        let mut map = GameMap::new(50,8,'#');
        map.create();
        map.setsize(40, 8);
        map.create();
    }
    #[test]
    fn changedelimiter() {
        let mut map = GameMap::new(50,8,'#');
        map.create();
        map.setdelimiter('*');
        map.create();
    }
}
