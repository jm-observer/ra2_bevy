//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{asset::LoadState, prelude::*};
use ra2_asset::{
    asset::{IniAsset, ShpAsset, TileAsset},
    loader::{IniFileAssetLoader, PaletteLoader, TilesAssetLoader},
    DebugGameState
};
use ra2_bin::add_assets_and_loaders;
use ra2_data::color::Palette;
use ra2_plugins::cursor::{init_cursor, CursorComponent, CursorShp};
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    add_assets_and_loaders(&mut app);
    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            init_cursor.run_if(in_state(DebugGameState::Loading))
        )
        .add_systems(
            Update,
            check_cursor_init.run_if(in_state(DebugGameState::Loading))
        )
        .add_systems(
            Update,
            move_mouse_events_system.run_if(in_state(DebugGameState::PlayTime))
        )
        .run();
}

/// set up a simple 3D scenew
fn setup(
    mut commands: Commands,
    assert_server: ResMut<AssetServer>,
    mut next_state: ResMut<NextState<DebugGameState>>
) {
    let cursor = CursorShp::new(&assert_server);
    commands.insert_resource(cursor);
    commands.spawn(Camera2dBundle::default());
    next_state.set(DebugGameState::Loading);
}

fn check_cursor_init(
    mut cursor_shp: ResMut<CursorShp>,
    mut next_state: ResMut<NextState<DebugGameState>>
) {
    if cursor_shp.init {
        next_state.set(DebugGameState::PlayTime);
    }
}

fn move_mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut Transform, With<CursorComponent>>
) {
    let mut direction = Vec3::ZERO;
    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
        direction.x = event.position.x;
        direction.y = event.position.y;
    }
    if direction != Vec3::ZERO {
        let query = &mut query;
        for mut transform in query.into_iter() {
            transform.translation.x = direction.x;
            transform.translation.y = direction.y;
        }
    }
}
