mod camera;
mod cursor_keyboard;
mod window;

use bevy::prelude::{TextBundle, TextStyle, *};
pub use camera::*;
pub use cursor_keyboard::*;
pub use window::*;

/// 1. 计算、展示鼠标的相对坐标（相对显示屏），坐标
/// 2. 计算、更新、展示相机的坐标
/// 3. 展示显示屏的分辨率
pub struct CameraChangePlugin;

impl Plugin for CameraChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraPositionChangeEvent>()
            .add_event::<CursorPositionChangeEvent>()
            .add_systems(Startup, init)
            .add_systems(Update, update_cursor_relative_position)
            .add_systems(Update, update_camera_position_by_cursor)
            .add_systems(Update, update_camera_position_by_keyboard)
            .add_systems(Update, on_create_system)
            .add_systems(Update, on_resize_system)
            .add_systems(Update, update_cursor_position)
            .add_systems(Update, update_camera_position);
    }
}

pub fn init(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    let camera_position = camera.transform.clone();
    let camaera_position_text = format!(
        "Camera Positon x: {} y: {} z: {}",
        camera_position.translation.x, camera_position.translation.y, camera_position.translation.z
    );

    commands.insert_resource(CursorRelativePosition::default());
    commands.insert_resource(CursorPosition::default());
    commands.spawn(camera).insert(Camera);
    commands
        .spawn(
            TextBundle::from_section(
                camaera_position_text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            })
        )
        .insert(CameraPositionText);

    window::init(&mut commands);
    cursor_keyboard::init(&mut commands);
}
