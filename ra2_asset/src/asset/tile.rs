use bevy::{prelude::*, reflect::TypeUuid};
use ra2_data::{color::Bitmap, tile::TileFile};
use std::sync::Arc;

#[derive(Clone)]
pub struct TileTexture {
    // 不能去掉，后续在渲染时，有用到里面的has_extrate
    pub file:   Arc<TileFile>,
    // pub has_damaged_data: Vec<bool>,
    pub images: Vec<Handle<Image>>
}

#[derive(TypeUuid, Clone)]
#[uuid = "b5e27e07-6a34-4af9-bed8-189b7af170ef"]
pub struct TileAsset {
    pub name:   String,
    /// 便于后续的clone
    pub file:   Arc<TileFile>,
    // pub has_damaged_data: Vec<bool>,
    pub images: Vec<Arc<Bitmap>> // pub file: Arc<TmpFileData>,
}

impl TileAsset {
    pub fn from_file(tf: TileFile, name: String) -> Self {
        let mut images = Vec::<Arc<Bitmap>>::with_capacity(tf.images.len());
        // let mut has_damaged_data = Vec::<bool>::with_capacity(tf.images.len());
        let block_width = tf.block_width;
        let block_height = tf.block_height;
        for image in &tf.images {
            images.push(Arc::new(image.draw(block_width, block_height)));
        }
        let file = Arc::new(tf);
        Self {
            file,
            images,
            name // has_damaged_data,
        }
    }
}
