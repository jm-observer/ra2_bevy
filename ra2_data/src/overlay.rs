#[derive(Clone, Debug)]
pub struct Overlay {
    pub id:    i32,
    pub rx:    i32,
    pub ry:    i32,
    pub value: u32
}
impl Overlay {
    pub fn new(rx: i32, ry: i32, id: i32, value: u32) -> Self {
        Self { id, rx, ry, value }
    }
}
