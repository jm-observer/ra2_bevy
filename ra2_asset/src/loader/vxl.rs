use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture
};
use ra2_data::vxl::VxlFileOrigin;

use crate::{asset::VxlFile, loader::get_file_name};

#[derive(Default)]
pub struct VxlAssetLoader;

impl AssetLoader for VxlAssetLoader {
    fn load<'b>(
        &'b self,
        bytes: &'b [u8],
        load_context: &'b mut LoadContext
    ) -> bevy::utils::BoxedFuture<'b, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let name = get_file_name(&load_context)?;
            let val: VxlFileOrigin = serde_json::from_slice(bytes).unwrap();
            let asset = VxlFile::deal(val, name);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vxl"]
    }
}
