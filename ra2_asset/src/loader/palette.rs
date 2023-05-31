use bevy::{asset::{AssetLoader, LoadedAsset}, log::error};

use crate::asset::Palette;

pub struct PaletteLoader;

impl AssetLoader for PaletteLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            match serde_json::from_slice::<Vec<u8>>(bytes) {
                Ok(val) => load_context
                    .set_default_asset(LoadedAsset::new(Palette::new_from_array(val.as_slice()))),
                Err(err) => {
                    error!("load palette fail: {:?}", err);
                    panic!("load palette fail")
                }
            }
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pal"]
    }
}
