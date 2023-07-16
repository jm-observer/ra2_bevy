//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_asset::{
    asset::TileAsset,
    loader::{PaletteLoader, TilesAssetLoader}
};
use ra2_data::color::Palette;
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<Palette>()
        .add_asset_loader(PaletteLoader)
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scenew
fn setup(mut commands: Commands, assert_server: ResMut<AssetServer>) {
    let palette = assert_server.load("palettes/isotem.pal");
    commands.insert_resource(CustomRes {
        palette,
        printed: false
    });
    commands.spawn(Camera2dBundle::default());
}

fn print_on_load(mut state: ResMut<CustomRes>, palette_assets: ResMut<Assets<Palette>>) {
    let palette_asset = palette_assets.get(&state.palette);
    if state.printed {
        return;
    }
    let Some(palette) = palette_asset else {
        return;
    };
    for color in &palette.colors {
        println!("{} {} {}", color.r, color.g, color.b);
    }
    state.printed = true;
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette: Handle<Palette>,
    pub printed: bool
}
