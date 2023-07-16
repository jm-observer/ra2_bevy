//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{asset::LoadState, prelude::*};
use ra2_asset::{
    asset::{IniAsset, TileAsset},
    loader::{IniFileAssetLoader, PaletteLoader, TilesAssetLoader}
};
use ra2_data::color::Palette;
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<IniAsset>()
        .add_asset_loader(IniFileAssetLoader)
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scenew
fn setup(mut commands: Commands, assert_server: ResMut<AssetServer>) {
    let palette = assert_server.load("ini/temperate.ini");
    commands.insert_resource(CustomRes {
        ini:     palette,
        printed: false
    });
    commands.spawn(Camera2dBundle::default());
}

fn print_on_load(
    mut state: ResMut<CustomRes>,
    palette_assets: ResMut<Assets<IniAsset>>,
    asset_server: Res<AssetServer>
) {
    let palette_asset = palette_assets.get(&state.ini);
    if state.printed {
        return;
    }
    let Some(palette) = palette_asset else {
        return;
    };
    let temperate_ini_status = asset_server.get_load_state(&state.ini) == LoadState::Loaded;
    println!("{} {:?}", temperate_ini_status, palette.0);
    state.printed = true;
}

#[derive(Resource)]
pub struct CustomRes {
    pub ini:     Handle<IniAsset>,
    pub printed: bool
}
