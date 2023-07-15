use crate::map_object::{MapObject, ObjectType};

/// 地形、地势
#[derive(Clone)]
pub struct Terrain;

impl Terrain {
    pub fn new(name: String, rx: i32, ry: i32) -> MapObject {
        let mut mo = MapObject::new(name, ObjectType::Terrain, rx, ry, None);
        mo.terrain = Some(Terrain);
        mo
    }
}
