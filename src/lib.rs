pub mod maps;
pub mod entities;
use maps::map::*;
use entities::entity::*;


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
    #[test]
    fn entities() {
        let mut map = GameMap::new(50, 8, '#');
        let entities: Vec<Entity> = vec![Entity::new(10, 3, false, '8', false),Entity::new(15, 3, false, '$', false)];
        map.drawentities(entities);
    }
}
