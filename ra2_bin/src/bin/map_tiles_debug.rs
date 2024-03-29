//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{asset::LoadState, prelude::*};
use ra2_asset::{
    asset::{IniAsset, MapAsset, TileAsset, TileTexture},
    loader::{IniFileAssetLoader, MapLoader, PaletteLoader, TilesAssetLoader},
    DebugGameState
};
use ra2_bin::add_assets_and_loaders;
use ra2_data::{color::Palette, rule::GeneralRules};
use ra2_plugins::cursor_keyboard_camera::CameraChangePlugin;
use ra2_render::{
    data::map::{MapTileCollection, TileCollection},
    system::map::create_map_tile_sprites
};
use std::{collections::HashMap, env, sync::Arc};

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(CameraChangePlugin);
    add_assets_and_loaders(&mut app);
    app.add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut next_state: ResMut<NextState<DebugGameState>>
) {
    let palette = asset_server.load("palettes/isotem.pal");
    let tiles = asset_server.load_folder("tile/tiles.tem").unwrap();
    let temperate_ini = asset_server
        .load::<IniAsset, &str>("ini/temperate.ini")
        .into();

    let rule_ini = asset_server.load::<IniAsset, &str>("ini/rules.ini").into();
    let map = asset_server.load::<MapAsset, &str>("map/mp02t2.map");

    commands.insert_resource(CustomRes {
        tiles,
        palette,
        rule_ini,
        map,
        temperate_ini,
        printed: false
    });
    // let mut orb = Camera2dBundle::default();
    // orb.transform = Transform::from_xyz(2500.0, -1200.0, 999.9);
    // commands.spawn(orb).insert(ra2_bin::Camera);
    next_state.set(DebugGameState::Loading);
}

fn print_on_load(
    mut commands: Commands,
    mut state: ResMut<CustomRes>,
    tiles: ResMut<Assets<TileAsset>>,
    pals: ResMut<Assets<Palette>>,
    inis: ResMut<Assets<IniAsset>>,
    maps: ResMut<Assets<MapAsset>>,
    mut asset_textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>
) {
    if state.printed {
        return;
    }
    let tiles_status = asset_server
        .get_group_load_state(state.tiles.iter().map(|handle| handle.id()))
        == LoadState::Loaded;
    let rule_ini_status = asset_server.get_load_state(&state.rule_ini) == LoadState::Loaded;
    let palette_status = asset_server.get_load_state(&state.palette) == LoadState::Loaded;
    let temperate_ini_status =
        asset_server.get_load_state(&state.temperate_ini) == LoadState::Loaded;
    let map_status = asset_server.get_load_state(&state.map) == LoadState::Loaded;
    if !(tiles_status && rule_ini_status && palette_status && temperate_ini_status && map_status) {
        return;
    }
    let pal = pals.get(&state.palette).unwrap();
    let temperate_ini = inis.get(&state.temperate_ini).unwrap();
    let rule_ini = inis.get(&state.rule_ini).unwrap();
    let map = maps.get(&state.map).unwrap();

    let rule = Arc::new(GeneralRules::read_ini(
        rule_ini.get_section("General").unwrap()
    ));

    let textures: HashMap<String, TileTexture> = state
        .tiles
        .iter()
        .map(|x| {
            let asset = tiles.get(&x.typed_weak()).unwrap();
            let images = asset
                .images
                .iter()
                .map(|x| {
                    let bitmap: Image = x.indexed_to_rgba(pal).unwrap().into();
                    asset_textures.add(bitmap)
                })
                .collect::<Vec<Handle<Image>>>();
            (
                asset.name.clone(),
                TileTexture {
                    file: asset.file.clone(),
                    images
                }
            )
        })
        .collect();
    let tcr = Arc::new(TileCollection::new(
        temperate_ini.0.clone(),
        ".tile".to_string(),
        &textures
    ));

    let tiles = MapTileCollection::init(&map.tiles, tcr, rule);
    create_map_tile_sprites(&mut commands, Arc::new(tiles));
    state.printed = true;
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette:       Handle<Palette>,
    pub temperate_ini: Handle<IniAsset>,
    pub rule_ini:      Handle<IniAsset>,
    pub map:           Handle<MapAsset>,
    pub tiles:         Vec<HandleUntyped>,
    pub printed:       bool
}
