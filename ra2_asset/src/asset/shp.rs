use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::TextureFormat};
use ra2_data::{
    color::{Bitmap, Palette},
    shp::{ShpFile, ShpImage}
};
use std::sync::Arc;

#[derive(TypeUuid, Clone)]
#[uuid = "1c2d46a6-ee75-4a52-b2e5-a7e7311eae11"]
pub struct ShpAsset {
    pub name:       String,
    pub height:     i32,
    pub width:      i32,
    pub num_images: i32,
    pub shp:        Vec<Arc<ShpImageAsset>>
}

impl ShpAsset {
    pub(crate) fn from_shp_file(sf: ShpFile, name: String) -> Result<Self> {
        let mut shp: Vec<Arc<ShpImageAsset>> = Vec::with_capacity(sf.images.len());
        for x in sf.images {
            shp.push(Arc::new(x.try_into()?));
        }
        Ok(Self {
            name,
            height: sf.height,
            width: sf.width,
            num_images: sf.num_images,
            shp
        })
    }

    pub fn gen_texture_atlas(
        &self,
        textures: &mut Assets<Image>,
        pal: &Palette,
        texture_atlases: &mut Assets<TextureAtlas>
    ) -> Result<Handle<TextureAtlas>> {
        let mut texture_atlas_builder =
            TextureAtlasBuilder::default().format(TextureFormat::Rgba8Unorm);
        let mut first_handle: Option<Handle<Image>> = None;
        let mut inited = true;
        for asset in &self.shp {
            let mut bitmap = asset.data.clone();
            bitmap.update_from_palette(pal)?;
            let texture = bitmap.into();
            let handle = textures.add(texture);
            let tmp = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone(), tmp);
            if inited {
                first_handle = Some(handle);
                inited = false
            }
        }
        let texture_atlas = texture_atlas_builder.finish(textures).unwrap();
        info!(
            "the index is {}",
            texture_atlas
                .get_texture_index(&first_handle.as_ref().unwrap())
                .unwrap()
        );
        Ok(texture_atlases.add(texture_atlas))
    }
}
pub struct ShpImageAsset {
    pub x:      i32,
    pub y:      i32,
    pub height: u32,
    pub width:  u32,
    pub data:   Bitmap
}

impl TryFrom<ShpImage> for ShpImageAsset {
    type Error = anyhow::Error;

    fn try_from(si: ShpImage) -> Result<Self, Self::Error> {
        let mut bitmap = Bitmap::rgba_bitmap(si.width, si.height);
        if let Some(data) = si.image_data {
            let data = STANDARD.decode(&data).unwrap();
            let index = Bitmap::indexed_bitmap_by_data(si.width, si.height, data)?;
            bitmap.draw_indexed_image(&index, 0, 0);
        }
        Ok(Self {
            x:      si.x,
            y:      si.y,
            height: si.height,
            width:  si.width,
            data:   bitmap
        })
    }
}
