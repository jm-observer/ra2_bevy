use crate::cursor_keyboard_camera::CameraPositionChangeEvent;
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{
        CursorMoved, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource, Window
    }
};

#[derive(Debug, Resource, Default)]
pub struct CursorPosition {
    x:              f32,
    y:              f32,
    on_left_edge:   bool,
    on_right_edge:  bool,
    on_up_edge:     bool,
    on_bottom_edge: bool
}

pub fn update_camera_position_by_cursor(
    position: Res<CursorPosition>,
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

pub fn update_cursor_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window>,
    mut position: ResMut<CursorPosition>
) {
    if cursor_moved_events.len() == 0 {
        return;
    }
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

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
