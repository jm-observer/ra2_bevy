//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_asset::{asset::PaletteAsset, loader::PaletteLoader};
use ra2_data::lighting::Lighting;
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<PaletteAsset>()
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

fn print_on_load(mut state: ResMut<CustomRes>, palette_assets: ResMut<Assets<PaletteAsset>>) {
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
    pub palette: Handle<PaletteAsset>,
    pub printed: bool
}

fn mp02t2_lighting() -> Lighting {
    Lighting {
        level:      0.008,
        ambient:    0.85,
        red:        1.0,
        green:      1.0,
        blue:       1.10,
        ground:     0.0,
        force_tint: true
    }
}
