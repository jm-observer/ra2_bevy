use crate::asset::MapAsset;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    log::error
};
use ra2_data::map::MapFile;
use serde_json::Value;
use std::sync::Arc;

pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            match serde_json::from_slice::<Value>(bytes) {
                Ok(val) => {
                    let file = MapFile::init(&val)?;
                    load_context.set_default_asset(LoadedAsset::new(MapAsset(Arc::new(file))));
                },
                Err(err) => {
                    error!("load palette fail: {:?}", err);
                    panic!("load palette fail")
                }
            }
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}
