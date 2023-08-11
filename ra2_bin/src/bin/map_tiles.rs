//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{asset::LoadState, prelude::*, window::WindowMode};
use ra2_asset::{
    asset::{IniAsset, MapAsset, PaletteAsset, TileAsset, TileTexture},
    loader::{IniFileAssetLoader, MapLoader, PaletteLoader, TilesAssetLoader}
};
use ra2_bin::mp02t2_lighting;
use ra2_data::{
    color::IsoPalettes,
    coord::{ISO_TILE_HEIGHT, ISO_TILE_SIZE},
    rule::GeneralRules
};
use ra2_plugins::cursor_keyboard_camera::{CameraChangePlugin, CursorPosition};
use ra2_render::{
    data::map::{MapTileCollection, MapTileUnitComponent, TileCollection},
    system::map::{create_map_tile_sprites, TilePostion}
};
use std::{collections::HashMap, env, sync::Arc};

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            CameraChangePlugin
        ))
        .add_asset::<MapAsset>()
        .add_asset::<PaletteAsset>()
        .add_asset::<TileAsset>()
        .add_asset::<IniAsset>()
        .add_asset_loader(PaletteLoader)
        .add_asset_loader(MapLoader)
        .add_asset_loader(TilesAssetLoader)
        .add_asset_loader(IniFileAssetLoader)
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .add_systems(Update, update_by_mouse_event)
        .add_systems(Update, update_unselecting_transform)
        .add_systems(Update, update_selecting_transform)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let palette = asset_server.load("palettes/isotem.pal");
    let tiles = asset_server.load_folder("tile/tiles.tem").unwrap();
    let temperate_ini = asset_server
        .load::<IniAsset, &str>("ini/temperat.ini")
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

    commands.insert_resource(SelectInfo::default());
}

fn print_on_load(
    mut commands: Commands,
    mut state: ResMut<CustomRes>,
    tiles: ResMut<Assets<TileAsset>>,
    pals: ResMut<Assets<PaletteAsset>>,
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
    let palette = pals.get(&state.palette).unwrap();
    let temperate_ini = inis.get(&state.temperate_ini).unwrap();
    let rule_ini = inis.get(&state.rule_ini).unwrap();
    let map = maps.get(&state.map).unwrap();

    let rule = Arc::new(GeneralRules::read_ini(
        rule_ini.get_section("General").unwrap()
    ));
    // todo
    let lighting = mp02t2_lighting();
    let palettes = IsoPalettes::new(palette.datas.as_slice(), &lighting);
    let pal = palettes.palettes[18];

    let textures: HashMap<String, TileTexture> = state
        .tiles
        .iter()
        .map(|x| {
            let asset = tiles.get(&x.typed_weak()).unwrap();
            let images = asset
                .images
                .iter()
                .map(|x| {
                    let bitmap: Image = x.indexed_to_rgba(&pal).unwrap().into();
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
    pub palette:       Handle<PaletteAsset>,
    pub temperate_ini: Handle<IniAsset>,
    pub rule_ini:      Handle<IniAsset>,
    pub map:           Handle<MapAsset>,
    pub tiles:         Vec<HandleUntyped>,
    pub printed:       bool
}

#[derive(Resource, Default)]
pub struct SelectInfo {
    pub left_pressed: bool
}

#[derive(Component)]
pub struct TileSelected;
#[derive(Component)]
pub struct TileSelecting;
#[derive(Component)]
pub struct TileUnSelecting;

pub fn update_by_mouse_event(
    tiles: Query<(&Transform, &MapTileUnitComponent, &TilePostion, Entity)>,
    tile_selected: Query<Entity, With<TileSelected>>,
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut select_info: ResMut<SelectInfo>
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        select_info.left_pressed = false;
        return;
    }
    if select_info.left_pressed == true {
        // 已经计算过了，就不再计算了
        return;
    }
    select_info.left_pressed = true;
    let transform = cursor_position.0.clone();
    if let Some((tile, positon, entity)) = find_tile_in_region(
        &transform,
        ISO_TILE_SIZE as f32,
        ISO_TILE_HEIGHT as f32,
        tiles
    ) {
        for entity in tile_selected.iter() {
            commands
                .entity(entity)
                .remove::<TileSelected>()
                .insert(TileUnSelecting);
        }
        info!("select {:?}, {:?}", positon.0, tile.0);
        commands.entity(entity).insert(TileSelecting);
    }
}

pub fn update_unselecting_transform(
    mut tile_selected: Query<(&mut Transform, Entity), With<TileUnSelecting>>,
    mut commands: Commands
) {
    for (mut transform, entity) in tile_selected.iter_mut() {
        transform.translation.y = transform.translation.y - 20.0;
        commands.entity(entity).remove::<TileUnSelecting>();
    }
}

pub fn update_selecting_transform(
    mut tile_selected: Query<(&mut Transform, Entity), With<TileSelecting>>,
    mut commands: Commands
) {
    for (mut transform, entity) in tile_selected.iter_mut() {
        transform.translation.y = transform.translation.y + 20.0;
        commands
            .entity(entity)
            .remove::<TileSelecting>()
            .insert(TileSelected);
    }
}

fn find_tile_in_region(
    point: &Vec2,
    half_width: f32,
    half_heigth: f32,
    tiles: Query<(&Transform, &MapTileUnitComponent, &TilePostion, Entity)>
) -> Option<(MapTileUnitComponent, TilePostion, Entity)> {
    let max_x = point.x + half_width;
    let min_x = point.x - half_width;
    let max_y = point.y + half_heigth;
    let min_y = point.y - half_heigth;
    for (tran, mtuc, position, entity) in tiles.iter() {
        if tran.translation.x >= min_x
            && tran.translation.x <= max_x
            && tran.translation.y >= min_y
            && tran.translation.y <= max_y
        {
            return Some((mtuc.clone(), position.clone(), entity));
        }
    }
    None
}
