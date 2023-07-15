#[derive(Copy, Clone)]
pub struct RaSize {
    pub x:      i32,
    pub y:      i32,
    pub width:  i32,
    pub height: i32
}

impl RaSize {
    pub fn init() -> RaSize {
        RaSize {
            x:      0,
            y:      0,
            width:  0,
            height: 0
        }
    }
}
