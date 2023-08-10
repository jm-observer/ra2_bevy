use crate::{
    color::{Bitmap, RaColor},
    pub_use::*
};
use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Serialize, Deserialize)]
pub struct TileFile {
    pub width:        u32,
    pub height:       u32,
    pub block_width:  u32,
    pub block_height: u32,
    pub images:       Vec<TileImage>
}

//融合了原有的TmpDrawable

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct TileImage {
    pub x:                 i32,
    pub y:                 i32,
    pub extra_x:           i32,
    pub extra_y:           i32,
    pub extra_width:       i32,
    pub extra_height:      i32,
    pub has_extra_data:    bool,
    pub has_damaged_data:  bool,
    pub has_z_data:        bool,
    pub height:            i32,
    pub terrain_type:      i32,
    pub ramp_type:         i32,
    pub radar_red_left:    i32,
    pub radar_green_left:  i32,
    pub radar_blue_left:   i32,
    pub radar_red_right:   i32,
    pub radar_green_right: i32,
    pub radar_blue_right:  i32,
    pub tile_data:         String,
    pub extra_data:        Option<String>,
    pub z_data:            Option<String>,
    pub extra_z_data:      Option<String>,
    radar_left:            Option<RaColor>,
    radar_right:           Option<RaColor> /* rel_tile_data: Option<Vec<u8>>,
                                            * rel_extra_data: Option<Vec<u8>>,
                                            * rel_z_data: Option<Vec<u8>>,
                                            * rel_extra_z_data: Option<Vec<u8>> */
}

impl TileImage {
    pub fn get_tile_data(&self) -> Vec<u8> {
        STANDARD.decode(&self.tile_data).unwrap()
    }

    pub fn get_extra_data(&self) -> Vec<u8> {
        STANDARD.decode(self.extra_data.as_ref().unwrap()).unwrap()
    }

    pub fn n(num: i32) -> i32 {
        if num < 0 { num + 256 } else { num }
    }

    pub fn init_radar(&mut self) {
        let r = Self::n(self.radar_red_left) as u8;
        let g = Self::n(self.radar_green_left) as u8;
        let b = Self::n(self.radar_blue_left) as u8;
        self.radar_left = Some(RaColor { r, g, b });

        let r = Self::n(self.radar_red_right) as u8;
        let g = Self::n(self.radar_green_right) as u8;
        let b = Self::n(self.radar_blue_right) as u8;
        self.radar_right = Some(RaColor { r, g, b });
    }

    pub fn get_radar_left(&self) -> &RaColor {
        self.radar_left.as_ref().unwrap()
    }

    pub fn get_radar_right(&self) -> &RaColor {
        self.radar_right.as_ref().unwrap()
    }

    ///返回indexedBitmap
    pub fn draw(&self, block_width: u32, block_height: u32) -> Bitmap {
        let mut tmp_block_width = block_width;
        let mut tmp_block_height = block_height;
        let mut tmp_a = 0;
        let mut tmp_o = 0;

        if self.has_extra_data {
            let x_extrax = 0.max(self.x - self.extra_x) as u32;
            let y_extray = 0.max(self.y - self.extra_y) as u32;
            tmp_a += x_extrax;
            tmp_o += y_extray;
            tmp_block_width += x_extrax;
            tmp_block_height += y_extray;
        }

        let mut bit_map = Bitmap::indexed_bitmap(tmp_block_width, tmp_block_height);
        self.draw_tile_block(&mut bit_map, block_width, block_height, tmp_a, tmp_o);
        if self.has_extra_data {
            self.draw_extra_data(&mut bit_map);
        }
        bit_map
    }

    fn draw_tile_block(
        &self,
        t: &mut Bitmap,
        block_width: u32,
        block_height: u32,
        col_index: u32,
        row_index: u32
    ) {
        let i = block_width;
        let n = block_height;
        let a = t.data.as_mut_slice();
        let rel_tile_data = self.get_tile_data();
        let o = n / 2;
        //更新起始点=中间点-2 + （额外参数）
        let mut update_pixel = (i / 2 - 2 + t.width * row_index + col_index) as usize;
        //总像素点数
        let all_pixel_amount = (t.width * t.height) as usize;
        //tile_data数组的index
        let mut tile_data_index = 0usize;
        //u=更新的第几行数
        let mut update_row = 0;
        //每行需要更新的像素数量
        let mut update_row_pixel_amount = 0;

        //上半段：正三角形
        while update_row < o {
            //d=每行的像素点数=上一行的像素点数+4
            update_row_pixel_amount = update_row_pixel_amount + 4;
            // println!(
            //     "update_row_start_pixel={} update_pixel={} update_row_pixel_amount={}
            // update_row={} tile_data_index={}",     update_pixel - (t.width *
            // update_row) as usize,     update_pixel,
            //     update_row_pixel_amount,
            //     update_row,
            //     tile_data_index
            // );
            let mut t_index = 0;
            while t_index < update_row_pixel_amount {
                let val = rel_tile_data[tile_data_index];
                if 0 != val && update_pixel < all_pixel_amount {
                    a[update_pixel] = val;
                }
                update_pixel += 1;
                tile_data_index += 1;
                t_index += 1;
            }
            //每行的起始像素位置=上一行的起始像素位置(l - d)-2
            //每行的起始像素在总数组的位置=每行的起始像素位置 + 宽度t.width
            // l = l + t.width as usize - (d + 2) as usize;

            update_pixel = update_pixel - update_row_pixel_amount + t.width as usize - 2usize;
            update_row += 1;
        }
        //更新下半部分图片：倒三角形
        update_pixel += 4;
        while update_row < n {
            update_row_pixel_amount -= 4;
            let mut tmp_t = 0;
            // println!(
            //     "update_row_start_pixel={} update_pixel={} update_row_pixel_amount={}
            // update_row={} tile_data_index={}",     update_pixel - (t.width *
            // update_row) as usize,     update_pixel,
            //     update_row_pixel_amount,
            //     update_row,
            //     tile_data_index
            // );
            while tmp_t < update_row_pixel_amount {
                let t = rel_tile_data[tile_data_index];
                if update_row_pixel_amount < all_pixel_amount {
                    a[update_pixel] = t;
                }
                update_pixel += 1;
                tile_data_index += 1;
                tmp_t += 1;
            }
            //每行的起始像素位置=上一行的起始像素位置(l - d)+2
            //每行的起始像素在总数组的位置=每行的起始像素位置 + 宽度t.width
            // println!(
            //     "update_row_start_pixel={} update_row={} tile_data_index={}",
            //     update_pixel - update_row_pixel_amount - (t.width * update_row) as usize,
            //     update_row,
            //     tile_data_index
            // );
            update_pixel = update_pixel - update_row_pixel_amount + t.width as usize + 2usize;
            update_row += 1;
        }
    }

    fn draw_extra_data(&self, bitmap: &mut Bitmap) {
        if !self.has_extra_data {
            return;
        }
        let i = bitmap.data.as_mut_slice();
        let n_width = bitmap.width;
        let s_height = bitmap.height;
        let r_x_max = 0.max(self.extra_x - self.x as i32) as usize;
        let a = n_width as usize;
        let all_pixel_amount = (n_width * s_height) as usize;
        //
        let mut l_update_piexl_index =
            (0 + a * 0.max(self.extra_y - self.y as i32) as usize + r_x_max) as usize;
        let mut extra_data_index = 0;
        let mut h = 0;
        let extra_data = self.get_extra_data();
        while h < self.extra_height {
            let mut t = 0;
            while t < self.extra_width {
                // println!("h={} t={} extra_data_index={}", h, t, extra_data_index);
                let extra_data_val = extra_data[extra_data_index];
                if 0 != extra_data_val && l_update_piexl_index < all_pixel_amount {
                    i[l_update_piexl_index] = extra_data_val;
                }
                l_update_piexl_index += 1;
                extra_data_index += 1;
                t += 1;
            }
            l_update_piexl_index = l_update_piexl_index + a - self.extra_width as usize;
            h = h + 1;
        }
    }
}
