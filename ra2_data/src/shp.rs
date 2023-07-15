use crate::pub_use::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShpFile {
    pub height:     i32,
    pub width:      i32,
    pub num_images: i32,
    pub images:     Vec<ShpImage>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ShpImage {
    pub x:          i32,
    pub y:          i32,
    pub height:     u32,
    pub width:      u32,
    pub image_data: Option<String> // image_data_encoded: Option<Vec<u8>>,
}
