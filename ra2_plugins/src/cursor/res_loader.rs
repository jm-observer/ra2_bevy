use crate::cursor::CursorComponent;
use bevy::{
    asset::{HandleId, LoadState},
    prelude::*
};
use ra2_asset::{asset::ShpAsset, DebugGameState};
use ra2_data::color::Palette;

#[derive(Resource)]
pub struct CursorShp {
    pub shp:          Handle<ShpAsset>,
    pub pal:          Handle<Palette>,
    pub asset_server: AssetServer,
    pub init:         bool
}
impl CursorShp {
    pub fn new(asset_server: &AssetServer) -> Self {
        let shp = asset_server.load("shp/ui/mouse.shp");
        let id: HandleId = "shp/ui/mouse.shp".into();
        println!("shp.id={:?}, id={:?}", shp.id(), id);
        let pal = asset_server.load("palettes/mousepal.pal").into();
        Self {
            shp,
            pal,
            init: false,
            asset_server: asset_server.clone()
        }
    }
}

pub fn init_cursor(
    mut cursor_shp: ResMut<CursorShp>,
    pal_assets: Res<Assets<Palette>>,
    shp_assets: Res<Assets<ShpAsset>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    if !(cursor_shp.asset_server.get_load_state(&cursor_shp.pal) == LoadState::Loaded
        && cursor_shp.asset_server.get_load_state(&cursor_shp.shp) == LoadState::Loaded)
    {
        return;
    }
    if cursor_shp.init {
        return;
    }
    info!("CursorShpStatus.init");
    let pal = pal_assets.get(&cursor_shp.pal).unwrap();
    let shp = shp_assets.get(&cursor_shp.shp).unwrap();

    let atlas_handle = shp
        .gen_texture_atlas(&mut textures, pal, &mut texture_atlases)
        .unwrap();

    commands
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(CursorComponent);
    cursor_shp.init = true;
}
