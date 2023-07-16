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

/// 宽高。程序屏幕的大小
#[derive(Copy, Clone, Debug)]
pub struct WHSize {
    pub width:  i32,
    pub height: i32
}
impl WHSize {
    pub fn init() -> Self {
        WHSize {
            width:  0,
            height: 0
        }
    }

    pub fn new(width: i32, height: i32) -> Self {
        WHSize { width, height }
    }
}
