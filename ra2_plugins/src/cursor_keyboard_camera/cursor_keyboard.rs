use crate::cursor_keyboard_camera::CameraPositionChangeEvent;
use bevy::{input::Input, math::Vec3, prelude::*};

/// 鼠标的相对window的位置，左上角为（0,0），左手坐标系
#[derive(Debug, Resource, Default)]
pub struct CursorRelativePosition {
    x:              f32,
    y:              f32,
    on_left_edge:   bool,
    on_right_edge:  bool,
    on_up_edge:     bool,
    on_bottom_edge: bool
}

/// 鼠标的坐标
#[derive(Debug, Resource, Default)]
pub struct CursorPosition(pub Vec2);

#[derive(Default, Event)]
pub struct CursorPositionChangeEvent;

#[derive(Component)]
pub struct CursorPositionText;

pub fn init(commands: &mut Commands) {
    let window_text = "Cursor Position:".to_string();
    commands
        .spawn(
            TextBundle::from_section(
                window_text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(70.0),
                left: Val::Px(10.0),
                ..default()
            })
        )
        .insert(CursorPositionText);
}

pub fn update_camera_position_by_cursor(
    position: Res<CursorRelativePosition>,
    mut events: EventWriter<CameraPositionChangeEvent>
) {
    if position.on_up_edge
        || position.on_bottom_edge
        || position.on_right_edge
        || position.on_left_edge
    {
        let mut event = CameraPositionChangeEvent::default();
        if position.on_left_edge {
            event.0 = -50.0;
        } else if position.on_right_edge {
            event.0 = 50.0;
        }
        if position.on_up_edge {
            event.1 = -50.0;
        } else if position.on_bottom_edge {
            event.1 = 50.0;
        }
        events.send(event);
    }
}

pub fn update_camera_position_by_keyboard(
    input: Res<Input<KeyCode>>,
    mut events: EventWriter<CameraPositionChangeEvent>
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize() * 10.0;
        events.send(CameraPositionChangeEvent(
            direction.x,
            direction.y,
            direction.z
        ));
    }
}

pub fn update_cursor_relative_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window>,
    mut position: ResMut<CursorRelativePosition>,
    mut events: EventWriter<CursorPositionChangeEvent>
) {
    if cursor_moved_events.len() == 0 {
        return;
    }
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();
    events.send(CursorPositionChangeEvent);

    for event in cursor_moved_events.iter() {
        position.x = event.position.x;
        position.y = event.position.y;

        if event.position.x == 0.0 {
            position.on_left_edge = true;
            position.on_right_edge = false;
        } else if event.position.x > width - 2.0 {
            position.on_right_edge = true;
            position.on_left_edge = false;
        } else {
            position.on_right_edge = false;
            position.on_left_edge = false;
        }
        if event.position.y == 0.0 {
            position.on_up_edge = true;
            position.on_bottom_edge = false;
        } else if event.position.y > height - 2.0 {
            position.on_bottom_edge = true;
            position.on_up_edge = false;
        } else {
            position.on_bottom_edge = false;
            position.on_up_edge = false;
        }
    }
}

pub fn update_cursor_position(
    events: EventReader<CursorPositionChangeEvent>,
    cursor_relative_position: Res<CursorRelativePosition>,
    camera_position: Query<(&GlobalTransform, &Camera), With<Camera>>,
    mut cursor_position_text: Query<&mut Text, With<CursorPositionText>>,
    mut cursor_position: ResMut<CursorPosition>
) {
    if events.len() == 0 {
        return;
    }
    let (global, camera) = camera_position.single();
    let Some(position) = camera.viewport_to_world_2d(
        global,
        Vec2::new(cursor_relative_position.x, cursor_relative_position.y)
    ) else {
        println!("viewport_to_world_2d: none");
        return;
    };

    let mut text = cursor_position_text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!("Cursor Position: x: {} y: {}", position.x, position.y,);
    cursor_position.0 = position;
}
