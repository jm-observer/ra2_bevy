use crate::map_object::{MapObject, ObjectType};
#[derive(Clone)]
pub struct Smudge;
impl Smudge {
    pub fn new(name: String, rx: i32, ry: i32) -> MapObject {
        let mut mo = MapObject::new(name, ObjectType::Smudge, rx, ry, None);
        mo.smudge = Some(Smudge);
        //todo
        mo
    }
}
