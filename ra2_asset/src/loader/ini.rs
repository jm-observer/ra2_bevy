use crate::{asset::IniAsset, loader::get_file_name};
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::debug,
    utils::BoxedFuture
};
use serde_json::Value;

/// 生成IniFile资产
#[derive(Default)]
pub struct IniFileAssetLoader;

impl AssetLoader for IniFileAssetLoader {
    fn load<'b>(
        &'b self,
        bytes: &'b [u8],
        load_context: &'b mut LoadContext
    ) -> BoxedFuture<'b, anyhow::Result<()>> {
        Box::pin(load(bytes, load_context))
    }

    fn extensions(&self) -> &[&str] {
        &["ini"]
    }
}

async fn load<'b>(bytes: &[u8], load_context: &mut LoadContext<'b>) -> Result<()> {
    let name = get_file_name(&load_context)?;
    let val: Value = serde_json::from_slice(bytes)?;
    debug!("IniFileAssetLoaderFile {} loading", name);
    load_context.set_default_asset(LoadedAsset::new(IniAsset::new(name, val, None)?));
    Ok(())
}
