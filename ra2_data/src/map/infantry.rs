use crate::map_object::{MapObject, ObjectType};

#[derive(Clone)]
pub struct Infantry {
    pub health:    i32,
    pub sub_cell:  i32,
    pub direction: i32
}
impl Infantry {
    pub fn new(
        name: String,
        rx: i32,
        ry: i32,
        health: i32,
        sub_cell: i32,
        direction: i32,
        on_bridge: bool,
        owner: String
    ) -> MapObject {
        let mut mo = MapObject::new(name, ObjectType::Infantry, rx, ry, Some(owner));
        mo.infantry = Some(Infantry {
            health,
            sub_cell,
            direction
        });
        mo.on_bridge = on_bridge;
        mo
    }
}
