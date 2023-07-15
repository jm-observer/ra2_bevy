use crate::{color::Palette, coord::Coord};
use anyhow::{bail, Result};
use bevy::{
    prelude::Image,
    render::texture::{CompressedImageFormats, ImageType}
};
use std::sync::Arc;

#[derive(Copy, Clone, Hash, Debug)]
pub enum PixelFormat {
    //RGB色彩模式是工业界的一种颜色标准，是通过对红(R)、绿(G)、
    // 蓝(B)三个颜色通道的变化以及它们相互之间的叠加来得到各式各样的颜色的，RGB即是代表红、绿、
    // 蓝三个通道的颜色，这个标准几乎包括了人类视力所能感知的所有颜色，是运用最广的颜色系统之一。
    Rgb     = 3,
    //RGBA是代表Red（红色）Green（绿色）Blue（蓝色）和Alpha的色彩空间
    Rgba    = 4,
    //索引色模式:网上和动画中常用的图像模式，当彩色图像转换为索引颜色的图像后包含近256种颜色。
    // 索引颜色图像包含一个颜色表。 如果直接存储每个像素的RGB颜色值，
    // 每个像素都需要占用3个字节。有些图像中有大量的色彩是相同的，
    // 那么就可以在图像文件中划出一个区域存放一个“调色板”来存储图像中的每一种颜色，
    // 然后在记录每个像素的时候，只是记录这个像素的颜色对应到“调色板”的第几号颜色。
    // 这种做法就称为索引像素格式，或者叫索引色，也有其他的叫法，大同小异。
    // 一般索引色格式的图片，常见的扩展名有GIF和PNG。而且调色板中最多不超过256中颜色，
    // 也可以缩减为128、64、32种色彩或者更少。
    Indexed = 1
}
// impl PixelFormat {
//     fn get_count(&self) -> i32 {
//         match self {
//             Rgb => {
//                 return 3;
//             }
//             Rgba => {
//                 return 4;
//             }
//             Indexed => {
//                 return 1;
//             }
//         }
//     }
// }
#[derive(Hash, Clone)]
pub struct Bitmap {
    pub data:         Vec<u8>,
    pub pixel_format: PixelFormat,
    pub width:        u32,
    pub height:       u32
}

impl From<Bitmap> for Image {
    fn from(bitmap: Bitmap) -> Self {
        let mut texture_data = Vec::<u8>::with_capacity(100);
        bitmap.bitmap_data_into_png_data(&mut texture_data);
        Image::from_buffer(
            texture_data.as_slice(),
            ImageType::Extension("png"),
            CompressedImageFormats::NONE,
            false
        )
        .unwrap()
    }
}
impl Bitmap {
    pub fn bitmap(width: u32, height: u32, pixel_format: PixelFormat) -> Self {
        let counts = width * height * pixel_format as u32;
        let data = vec![0u8; counts as usize];
        Bitmap {
            data,
            pixel_format,
            width,
            height
        }
    }

    pub fn indexed_bitmap_by_data(width: u32, height: u32, data: Vec<u8>) -> Result<Self> {
        let pixel_format = PixelFormat::Indexed;
        let counts = (width * height * pixel_format as u32) as usize;
        if counts as usize != data.len() {
            bail!("indexed_bitmap_by_data counts != data.len()");
        }
        Ok(Bitmap {
            data,
            pixel_format,
            width,
            height
        })
    }

    pub fn indexed_bitmap(width: u32, height: u32) -> Self {
        Self::bitmap(width, height, PixelFormat::Indexed)
    }

    pub fn rgb_bitmap(width: u32, height: u32) -> Self {
        Self::bitmap(width, height, PixelFormat::Rgb)
    }

    pub fn rgba_bitmap(width: u32, height: u32) -> Self {
        Self::bitmap(width, height, PixelFormat::Rgba)
    }

    pub fn indexed_to_rgba(&self, palette: &Palette) -> Result<Self> {
        if !matches!(self.pixel_format, PixelFormat::Indexed) {
            bail!("!matches!(self.pixel_format, PixelFormat::Indexed)");
        }

        let mut bitmap = Self::rgba_bitmap(self.width, self.height);
        //debug_draw_indexed_image
        bitmap.draw_indexed_image(self, 0, 0);
        bitmap.update_from_palette(palette);
        Ok(bitmap)
    }

    fn bitmap_data_into_png_data(&self, buffer_data: &mut Vec<u8>) {
        let datas: &[u8] = self.data.as_slice();
        let mut encoder = png::Encoder::new(buffer_data, self.width, self.height); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(datas).unwrap();
    }

    pub fn update_from_palette(&mut self, palette: &Palette) -> Result<()> {
        match self.pixel_format {
            PixelFormat::Indexed => {
                bail!("PixelFormat::Indexed should not be update_from_palette");
            },
            _ => {
                let interval = self.pixel_format as usize;
                let mut index = 0usize;

                let array = self.data.as_mut_slice();
                let lenth = array.len();
                let mut tmp: usize;
                let palette_index = 0usize;
                while index < lenth {
                    tmp = array[index] as usize;
                    // palette_index = (tmp * interval) as usize;
                    if tmp > 0 {
                        let color = palette.colors[tmp];
                        array[index] = color.r;
                        array[index + 1] = color.g;
                        array[index + 2] = color.b;
                        if interval == 4 {
                            array[index + 3] = 255;
                        }
                    }
                    index = index + interval;
                }
            }
        }
        Ok(())
    }

    pub fn update_from_shadow_palette(&mut self, palette: &Palette) -> Result<()> {
        match self.pixel_format {
            PixelFormat::Indexed => {
                bail!("PixelFormat::Indexed should not be update_from_shadow_palette");
            },
            _ => {
                let interval = self.pixel_format as usize;
                let mut index = 0usize;

                let array = self.data.as_mut_slice();
                let lenth = array.len();
                let mut tmp: usize;
                let palette_index = 0usize;
                while index < lenth {
                    tmp = array[index] as usize;
                    // palette_index = (tmp * interval) as usize;
                    if tmp > 0 {
                        let color = palette.colors[tmp];
                        array[index] = color.r;
                        array[index + 1] = color.g;
                        array[index + 2] = color.b;
                        if interval == 4 {
                            array[index + 3] = 150;
                        }
                    }
                    index = index + interval;
                }
            }
        }
        Ok(())
    }

    pub fn draw_indexed_image_from(&mut self, pip: Arc<Bitmap>, coord: Coord) {
        self.draw_indexed_image(pip.as_ref(), coord.x as u32, coord.y as u32)
    }

    ///将indexedBitmap转化成自身（rgb_bitmap/rgba_bitmap）的部分数据（根据t/i）
    /// 单纯将像素的头个值赋值、若存在第4个值则赋值为255
    ///i第几行(y)？t第几个(x)
    pub fn draw_indexed_image(&mut self, pip: &Bitmap, t: u32, i: u32) {
        let n = self.pixel_format as usize;
        let r = self.data.as_mut_slice();
        let a = self.width as usize;
        let pip_height = pip.height as usize;
        let pip_width = pip.width as usize;
        //每行的数值 数。1个像素等于n个数值
        let o = n * a;
        //总的像素=高*宽*颜色位数
        let l = n * a * (self.height as usize);
        let mut c = (0 + o * i as usize + n * t as usize) as usize;
        let mut h = 0;
        for s in 0..pip_height {
            for t in 0..pip_width {
                let data = *pip.data.get(h).unwrap();
                if data != 0u8 && c < l {
                    r[c] = data;
                    if n >= 3 {
                        r[c + 1] = 0;
                        r[c + 2] = 0;
                    }
                    if n == 4 {
                        r[c + 3] = 255;
                    }
                }
                c = c + n;
                h = h + 1;
            }
            c = c + o - pip_width * n;
        }
    }

    ///将indexedBitmap转化成自身（rgb_bitmap/rgba_bitmap）的部分数据（根据t/i）
    /// 单纯将像素的头个值赋值、若存在第4个值则赋值为255
    ///i第几行(y)？t第几个(x)
    pub fn debug_draw_indexed_image(&mut self, pip: &Bitmap, t: u32, i: u32) -> Result<()> {
        let n = self.pixel_format as usize;
        let r = self.data.as_mut_slice();
        let a = self.width as usize;
        let pip_height = pip.height as usize;
        let pip_width = pip.width as usize;
        //每行的数值 数。1个像素等于n个数值
        let o = n * a;
        //总的像素=高*宽*颜色位数
        let l = n * a * (self.height as usize);
        if pip.data.len() != a * (self.height as usize) {
            bail!("pip.data.len() != a * (self.height as usize)")
        }
        let mut c = (0 + o * i as usize + n * t as usize) as usize;
        let mut h = 0;
        let mut start_index: (usize, bool);
        let mut last_index: usize;
        for s in 0..pip_height {
            start_index = (0usize, false);
            last_index = 0usize;
            for t in 0..pip_width {
                let data = *pip.data.get(h).unwrap();
                if data != 0u8 && c < l {
                    if !start_index.1 {
                        start_index.0 = c;
                        start_index.1 = true;
                    }
                    last_index = c;
                    r[c] = data;
                    if n >= 3 {
                        r[c + 1] = 0;
                        r[c + 2] = 0;
                    }
                    if n == 4 {
                        r[c + 3] = 255;
                    }
                }

                c = c + n;
                h = h + 1;
            }
            // 每行第1个和最后1个，去掉色盘颜色，形成框框
            r[start_index.0] = 0;
            r[last_index] = 0;
            c = c + o - pip_width * n;
        }
        Ok(())
    }

    // drawIndexedImage(e, t, i) {
    // const n = s(this.pixelFormat)
    // , r = this.ra2.data
    // , a = this.width
    // , o = n * a
    // , l = n * a * this.height;
    // let c = 0 + o * i + n * t
    // , h = 0;
    // for (let s = 0; s < e.height; s++) {
    // for (let t = 0; t < e.width; t++) {
    // let t = e.ra2.data[h];
    // 0 !== t && 0 <= c && c < l && (r[c] = t,
    // n >= 3 && (r[c + 1] = 0,
    // r[c + 2] = 0),
    // 4 === n && (r[c + 3] = 255)),
    // c += n,
    // h++
    // }
    // c += o - e.width * n
    // }
    // }
}
