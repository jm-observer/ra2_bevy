use bevy::prelude::*;
use ra2_data::coord::{ISO_TILE_HEIGHT, ISO_TILE_SIZE};

#[derive(Resource)]
pub struct CursorShpRes {
    pub atlas_handle: Handle<TextureAtlas>
}
#[derive(Debug, Resource)]
pub struct CoordsBound {
    /// 光标的限制区域（左下角坐标, 右上角坐标组成一个长方形区域）
    cursor_left_down_vertex: Vec3,
    cursor_right_up_vertex:  Vec3,
    /// 光标的限制区域（左下角坐标, 右上角坐标组成一个长方形区域）
    // camera_left_down_vertex: Vec3,
    // camera_right_up_vertex: Vec3,
    /// 窗口的半宽度
    half_win_width:          f32,
    /// 窗口的半高度
    half_win_heigth:         f32,
    map_width:               f32,
    map_height:              f32,
    /// 在边框时，光标自动移动的位置
    auto_move_length:        f32,
    full_size_width:         i32
}

impl CoordsBound {
    pub fn init(
        half_win_width: f32,
        half_win_heigth: f32,
        map_width: f32,
        map_height: f32,
        full_size_width: i32
    ) -> Self {
        /// 地图左上角为坐标（0.0， 0.0， 0.0），还有一种地图左下角为（0.0，
        /// 0.0， 0.0） 为查看效果，矩形区域扩大
        let extend = 50.0;
        let cursor_left_down_vertex = Vec3::new(0.0 - extend, -1.0 * map_height - extend, 0.0);
        let cursor_right_up_vertex = Vec3::new(map_width + extend, 0.0 + extend, 0.0);
        // let camera_left_down_vertex = Vec3::new(cursor_left_down_vertex.x +
        // half_win_width, cursor_left_down_vertex.y + half_win_heigth, 0.0);
        // let camera_right_up_vertex = Vec3::new(cursor_right_up_vertex.x -
        // half_win_width, cursor_right_up_vertex.y - half_win_heigth, 0.0);
        Self {
            cursor_left_down_vertex,
            cursor_right_up_vertex,
            half_win_heigth,
            half_win_width,
            map_width,
            map_height,
            auto_move_length: 10.0,
            full_size_width
        }
    }

    pub fn tile_3d_to_world(rx: i32, ry: i32, z: i32) {}

    pub fn rxy_tran_to_dxy(&self, rx: i32, ry: i32) -> (i32, i32) {
        (
            rx - ry + self.full_size_width - 1,
            rx + ry - self.full_size_width - 1
        )
    }

    pub fn dxy_to_coords(dx: i32, dy: i32) -> (f32, f32) {
        (
            (dx * ISO_TILE_SIZE) as f32,
            (dy * ISO_TILE_HEIGHT * -1) as f32
        )
    }

    pub fn rxy_tran_to_coords(&self, rx: i32, ry: i32) -> (f32, f32) {
        let (dx, dy) = self.rxy_tran_to_dxy(rx, ry);
        Self::dxy_to_coords(dx, dy)
    }

    /// 若光标停留在窗口的边缘，则自动移动相机和光标位置
    pub fn auto_move_cursor(&self, cursor_translation: &mut Vec3, camera_translation: &Vec3) {
        let interval_x = (cursor_translation.x - camera_translation.x).abs() - self.half_win_width;
        if interval_x == 0.0 {
            if cursor_translation.x > camera_translation.x {
                cursor_translation.x += self.auto_move_length;
            } else {
                cursor_translation.x -= self.auto_move_length;
            }
        }
        let interval_y = (cursor_translation.y - camera_translation.y).abs() - self.half_win_heigth;
        if interval_y == 0.0 {
            if cursor_translation.y > camera_translation.y {
                cursor_translation.y += self.auto_move_length;
            } else {
                cursor_translation.y -= self.auto_move_length;
            }
        }
        self.adjust_cursor_translate(cursor_translation);
    }

    /// 相机随着光标移动
    pub fn adjust_camera_translation(
        &self,
        camera_translation: &mut Vec3,
        cursor_translation: &Vec3
    ) {
        // 移动相机
        let interval_x = (cursor_translation.x - camera_translation.x).abs() - self.half_win_width;
        if interval_x > 0.0 {
            if cursor_translation.x > camera_translation.x {
                camera_translation.x += interval_x;
            } else {
                camera_translation.x -= interval_x;
            }
        }
        let interval_y = (cursor_translation.y - camera_translation.y).abs() - self.half_win_heigth;
        // println!("befor cursor.y={} camera.y={}", cursor_translation.y,
        // camera_translation.y);
        if interval_y > 0.0 {
            if cursor_translation.y > camera_translation.y {
                camera_translation.y += interval_y;
            } else {
                camera_translation.y -= interval_y;
            }
        }
        // println!("after cursor.y={} camera.y={}", cursor_translation.y,
        // camera_translation.y);
    }

    pub fn adjust_cursor_translate(&self, cursor_translation: &mut Vec3) {
        // 不允许光标超出限制区域（地图）
        cursor_translation.x = cursor_translation.x.max(self.cursor_left_down_vertex.x);
        cursor_translation.x = cursor_translation.x.min(self.cursor_right_up_vertex.x);
        cursor_translation.y = cursor_translation.y.max(self.cursor_left_down_vertex.y);
        cursor_translation.y = cursor_translation.y.min(self.cursor_right_up_vertex.y);
        // println!("cursor.y={} cursor_left_down_vertex.y={}
        // cursor_right_up_vertex.y={}", cursor_translation.y,
        // self.cursor_left_down_vertex.y, self.cursor_right_up_vertex.y);
    }

    pub fn adjust_left_up_text_translate(
        &self,
        camera_translation: &Vec3,
        text_translation: &mut Vec3
    ) {
        text_translation.x = camera_translation.x - self.half_win_width + 350.0;
        text_translation.y = camera_translation.y + self.half_win_heigth - 100.0;
    }
}
