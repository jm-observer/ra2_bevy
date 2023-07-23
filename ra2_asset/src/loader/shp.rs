use crate::{asset::ShpAsset, loader::get_file_name};
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    log::error
};
use ra2_data::shp::ShpFile;

pub struct ShpLoader;

impl AssetLoader for ShpLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let name = get_file_name(load_context)?;
            match serde_json::from_slice::<ShpFile>(bytes) {
                Ok(val) => load_context
                    .set_default_asset(LoadedAsset::new(ShpAsset::from_shp_file(val, name)?)),
                Err(err) => {
                    error!("load shp fail: {:?}", err);
                    panic!("load shp fail")
                }
            }
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["shp"]
    }
}
