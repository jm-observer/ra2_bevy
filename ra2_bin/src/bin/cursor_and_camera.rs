//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_plugins::cursor_keyboard_camera::CameraChangePlugin;
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
    app.add_plugins(DefaultPlugins)
        .add_plugin(CameraChangePlugin)
        .run();
}
