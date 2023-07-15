use bevy::log::error;

pub const ISO_TILE_SIZE: i32 = 30;
pub const ISO_TILE_HEIGHT: i32 = 15;

///替代原来Coords
pub struct Coord {
    pub x: f32,
    pub y: f32
}

///坐标
pub struct Coords {
    x: f32,
    y: f32
}

impl Coords {
    pub fn init(x: f32, y: f32) -> Self {
        Coords { x, y }
    }

    pub fn world_to_screen(&self, t: f32, i: f32) -> Coord {
        let t = t - self.x;
        let i = i - self.y;
        Coord {
            x: t - i - 1.0,
            y: (t + i - 1.0) / 2.0
        }
    }

    pub fn screen_to_world(&self, t: i32, i: i32) -> Coord {
        let t = t as f32;
        let i = i as f32;
        Coord {
            x: (t + 2.0 * i) / 2.0 + 1.0 + self.x,
            y: (2.0 * i - t) / 2.0 + self.y
        }
    }

    pub fn tile_3d_to_world(rx: i32, ry: i32, z: i32) -> (f32, f32, f32) {
        let s = Self::tile_to_world(rx, ry);
        let r = Self::tile_height_to_world(z);
        // info!(
        //     "tile_3d_to_world: rx={} ry={} => ({}, {}, {})",
        //     rx, ry, s.x, r, s.y
        // );
        (s.x, r, s.y)
    }

    pub fn tile_height_to_world(t: i32) -> f32 {
        (t * ISO_TILE_SIZE / 2) as f32 * Self::get_z_scale() + 1.0
    }

    fn get_z_scale() -> f32 {
        // Math.cos(i.Engine.ISO_CAMERA_BETA) / Math.scale(i.Engine.ISO_CAMERA_ALPHA)
        0.8165
    }

    pub fn tile_to_world_offset(e: i32, t: i32, offset_x: i32, offset_y: i32) -> Coord {
        Coord {
            x: (e * ISO_TILE_SIZE + offset_x) as f32,
            y: (t * ISO_TILE_HEIGHT + offset_y) as f32
        }
    }

    pub fn tile_to_world(e: i32, t: i32) -> Coord {
        Coord {
            x: (e * ISO_TILE_SIZE) as f32,
            y: (t * ISO_TILE_SIZE) as f32
        }
    }

    //
    pub fn screen_to_screen_tile(e: f32, t: f32) -> Coord {
        Coord {
            x: (e / ISO_TILE_SIZE as f32) as f32,
            y: t / ((ISO_TILE_SIZE / 2) as f32)
        }
    }

    //
    // pub fn world_to_tile(e: i32, t: i32) -> Coord {
    //     Coord {
    //         x: e / ISO_TILE_SIZE,
    //         y: t / ISO_TILE_SIZE,
    //     }
    // }
    //
    pub fn tile_to_screen(&self, t: i32, i: i32) -> Coord {
        let n = Self::tile_to_world(t, i);
        self.world_to_screen(n.x, n.y)
    }

    //
    pub fn get_world_tile_size() -> i32 {
        return ISO_TILE_SIZE as i32;
    }

    pub fn screen_tile_to_screen(e: i32, t: i32) -> Coord {
        Coord {
            x: (e * ISO_TILE_SIZE) as f32,
            y: (t * ISO_TILE_HEIGHT) as f32
        }
    }

    pub fn vec_world_to_leptons() {
        error!("todo");
    }

    pub fn vec_world_to_screen() {
        error!("todo");
    }
}
