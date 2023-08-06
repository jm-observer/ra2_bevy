//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_plugins::cursor_and_camera::{
    init, on_create_system, update_camera_position, update_camera_position_by_cursor,
    update_camera_position_by_keyboard, update_cursor_position, CameraPositionChangeEvent
};
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    let mut app = App::new();
    // app.add_plugins(DefaultPlugins.set(WindowPlugin {
    //     primary_window: Some(Window {
    //         mode: WindowMode::BorderlessFullscreen,
    //         ..default()
    //     }),
    //     ..default()
    // }));
    app.add_plugins(DefaultPlugins);
    app.add_event::<CameraPositionChangeEvent>()
        .add_systems(Startup, init)
        .add_systems(Update, update_cursor_position)
        .add_systems(Update, update_camera_position_by_cursor)
        .add_systems(Update, update_camera_position_by_keyboard)
        .add_systems(Update, on_create_system)
        .add_systems(Update, update_camera_position)
        .run();
}
