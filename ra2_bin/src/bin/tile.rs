//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    prelude::*
};
use ra2_asset::{
    asset::{TileAsset},
    loader::{PaletteLoader, TilesAssetLoader}
};
use ra2_data::color::{Palette};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<TileAsset>()
        .add_asset::<Palette>()
        .add_asset_loader(PaletteLoader)
        .add_asset_loader(TilesAssetLoader)
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, assert_server: ResMut<AssetServer>) {
    // let vxl = assert_server.load("vxl/1tnk.vxl");
    // let vxl = assert_server.load("vxl/1tnkbarl.vxl");

    let tile = assert_server.load("../../resource/clat01.tile");

    let palette = assert_server.load("../../resource/isotem.pal");

    commands.insert_resource(CustomRes {
        tile,
        palette,
        printed: false
    });
    commands.spawn(Camera2dBundle::default());
}

fn print_on_load(
    mut commands: Commands,
    state: ResMut<CustomRes>,
    tile_assets: ResMut<Assets<TileAsset>>,
    palette_assets: ResMut<Assets<Palette>>,
    mut asset_textures: ResMut<Assets<Image>>
) {
    let tile_asset = tile_assets.get(&state.tile);
    let palette_asset = palette_assets.get(&state.palette);
    if state.printed {
        return;
    }
    let (Some(asset), Some(palette)) = (tile_asset, palette_asset) else {
        return;
    };
    let image = asset
        .images
        .iter()
        .map(|x| {
            let bitmap: Image = x.indexed_to_rgba(&palette).unwrap().into();
            asset_textures.add(bitmap)
        })
        .collect::<Vec<Handle<Image>>>()
        .remove(0);
    commands.spawn(SpriteBundle {
        texture: image,
        ..default()
    });
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette: Handle<Palette>,
    pub tile:    Handle<TileAsset>,
    pub printed: bool
}
