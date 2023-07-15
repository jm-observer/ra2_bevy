use crate::map_object::{MapObject, ObjectType};

#[derive(Clone)]
pub struct Aircraft {
    pub health:    i32,
    pub direction: i32
}

impl Aircraft {
    pub fn new(
        name: String,
        rx: i32,
        ry: i32,
        owner: String,
        health: i32,
        direction: i32
    ) -> MapObject {
        let mut mo = MapObject::new(name, ObjectType::Aircraft, rx, ry, Some(owner));
        let aircraft = Aircraft { health, direction };
        mo.aircraft = Some(aircraft);
        mo
    }
}
