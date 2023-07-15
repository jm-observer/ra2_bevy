use crate::countries::Countries;

pub struct Structure {
    pub name:   String,
    pub owner:  &'static str,
    pub rx:     i32,
    pub ry:     i32,
    pub health: i32
}

impl Structure {
    pub fn new(name: String, rx: i32, ry: i32, owner: &String, health: i32) -> Self {
        Self {
            health,
            name,
            rx,
            ry,
            owner: Countries::get_contry_name(&owner)
        }
    }
}
