use crate::{asset::TileAsset, loader::get_file_name};
use anyhow::Result;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

#[derive(Default)]
pub struct TilesAssetLoader;

impl AssetLoader for TilesAssetLoader {
    fn load<'b>(
        &'b self,
        bytes: &'b [u8],
        load_context: &'b mut LoadContext
    ) -> BoxedFuture<'b, Result<()>> {
        Box::pin(async move {
            let name = get_file_name(load_context)?;
            match serde_json::from_slice(bytes) {
                Ok(tmp) => {
                    let file = TileAsset::from_file(tmp, name);
                    load_context.set_default_asset(LoadedAsset::new(file));
                },
                Err(e) => {
                    println!("tiles name={} 加载失败：{:?}", name, e);
                }
            }
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tile"]
    }
}
