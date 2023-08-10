//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_asset::{
    asset::{PaletteAsset, TileAsset},
    loader::{PaletteLoader, TilesAssetLoader}
};
use ra2_bin::mp02t2_lighting;
use ra2_data::color::IsoPalettes;
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<TileAsset>()
        .add_asset::<PaletteAsset>()
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

    let tile = assert_server.load("tile/tiles.tem/Cliff28.tile");

    let palette = assert_server.load("palettes/isotem.pal");

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
    palette_assets: ResMut<Assets<PaletteAsset>>,
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

    let lighting = mp02t2_lighting();

    let palettes = IsoPalettes::new(palette.datas.as_slice(), &lighting);
    let palette = palettes.palettes[18];
    let images = asset
        .images
        .iter()
        .map(|x| {
            let bitmap: Image = x.indexed_to_rgba(&palette).unwrap().into();
            asset_textures.add(bitmap)
        })
        .collect::<Vec<Handle<Image>>>();
    for (index, image) in images.into_iter().enumerate() {
        commands.spawn(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(0.0 + 100.0 * index as f32, 0.0, 0.0),
            ..default()
        });
    }

    let palette = palettes.palettes[0];
    let images = asset
        .images
        .iter()
        .map(|x| {
            let bitmap: Image = x.indexed_to_rgba(&palette).unwrap().into();
            asset_textures.add(bitmap)
        })
        .collect::<Vec<Handle<Image>>>();
    for (index, image) in images.into_iter().enumerate() {
        commands.spawn(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(0.0 + 100.0 * index as f32, 100.0, 0.0),
            ..default()
        });
    }
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette: Handle<PaletteAsset>,
    pub tile:    Handle<TileAsset>,
    pub printed: bool
}
