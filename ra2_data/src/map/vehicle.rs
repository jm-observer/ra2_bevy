use crate::map_object::{MapObject, ObjectType};
#[derive(Clone)]
pub struct Vehicle {
    pub health:    i32,
    pub direction: i32
}

impl Vehicle {
    pub fn new(
        name: String,
        rx: i32,
        ry: i32,
        owner: String,
        health: i32,
        direction: i32,
        on_bridge: bool
    ) -> MapObject {
        let mut mo = MapObject::new(name, ObjectType::Vehicle, rx, ry, Some(owner));
        let vechicle = Vehicle { health, direction };
        mo.vehicle = Some(vechicle);
        mo.on_bridge = on_bridge;
        mo
    }
}
