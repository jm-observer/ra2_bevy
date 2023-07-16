use super::res::*;
use bevy::{asset::HandleId, prelude::*};
use ra2_asset::asset::ShpAsset;
use ra2_data::color::Palette;

pub type CursorShpStatus = CommonAssetLoader<CursorShp>;
pub struct CursorShp {
    pub shp: ShpCommonLoader,
    pub pal: PaletteCommonLoader
}
impl CursorShp {
    pub fn new(asset_server: &AssetServer) -> Self {
        let shp = asset_server.load("shp/ui/mouse.shp");
        let id: HandleId = "shp/ui/mouse.shp".into();
        println!("shp.id={:?}, id={:?}", shp.id(), id);
        let pal = asset_server.load("palettes/mousepal.pal").into();
        Self {
            shp: shp.into(),
            pal
        }
    }
}

impl Loading for CursorShp {
    fn loading(&mut self, server: &AssetServer) -> bool {
        self.pal.loading(server) && self.shp.loading(server)
    }
}
impl CursorShpStatus {
    pub fn new_status(server: &AssetServer) -> CursorShpStatus {
        Self::new(CursorShp::new(server))
    }

    pub fn init(
        &mut self,
        asset_server: &AssetServer,
        pal_assets: &Assets<Palette>,
        shp_assets: &Assets<ShpAsset>,
        textures: &mut Assets<Image>,
        commands: &mut Commands,
        texture_atlases: &mut Assets<TextureAtlas>
    ) {
        if self.loading(asset_server) && !self.is_inited() {
            info!("CursorShpStatus.init");
            let pal = pal_assets
                .get(&self.get_asset_ref().pal.1.cast_weak())
                .unwrap();
            let shp = shp_assets
                .get(&self.get_asset_ref().shp.1.cast_weak())
                .unwrap();

            let atlas_handle = shp.gen_texture_atlas(textures, pal, texture_atlases);
            commands.insert_resource(CursorShpRes { atlas_handle });
            self.set_inited();
        }
    }
}
