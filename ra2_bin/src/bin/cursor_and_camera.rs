//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, window::WindowMode};
use ra2_plugins::cursor_and_camera::{
    update_camera_position_by_cursor, update_camera_position_by_keyboard, update_cursor_position,
    Camera, CursorPosition
};
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }));
    app.add_systems(Startup, setup)
        .add_systems(Update, update_cursor_position)
        .add_systems(Update, update_camera_position_by_cursor)
        .add_systems(Update, update_camera_position_by_keyboard)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CursorPosition::default());
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}
