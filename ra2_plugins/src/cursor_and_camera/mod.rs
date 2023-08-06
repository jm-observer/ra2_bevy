use bevy::{
    input::mouse::MouseMotion,
    log::debug,
    prelude::{
        info, Component, CursorMoved, EventReader, Input, KeyCode, Query, Res, ResMut, Resource,
        Transform, Vec3, Window, With
    }
};

#[derive(Component)]
pub struct Camera;

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
    mut camera_position: Query<&mut Transform, With<Camera>>
) {
    // let position = position.single();
    let mut camera_position = camera_position.single_mut();
    if position.on_up_edge
        || position.on_bottom_edge
        || position.on_right_edge
        || position.on_left_edge
    {
        if position.on_left_edge {
            camera_position.translation.x = camera_position.translation.x - 50.0;
        } else if position.on_right_edge {
            camera_position.translation.x = camera_position.translation.x + 50.0;
        }

        if position.on_up_edge {
            camera_position.translation.y = camera_position.translation.y - 50.0;
        } else if position.on_bottom_edge {
            camera_position.translation.y = camera_position.translation.y + 50.0;
        }
        info!(
            "x={} y={}",
            camera_position.translation.x, camera_position.translation.y
        );
    }
}

pub fn update_camera_position_by_keyboard(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let query = &mut query;
    for mut transform in query.into_iter() {
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
            info!("{}", direction);
            transform.translation += direction.normalize() * 100.0;
        }
    }
}

pub fn update_cursor_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut windows: Query<&mut Window>,
    mut position: ResMut<CursorPosition>
) {
    if cursor_moved_events.len() > 0 {
        let window = windows.single();
        let width = window.physical_width() as f32;
        let height = window.physical_height() as f32;

        for event in cursor_moved_events.iter() {
            position.x = event.position.x;
            position.y = event.position.y;
            info!(
                "{} {} {} {} {} {}",
                width,
                height,
                window.resolution.width(),
                window.resolution.height(),
                position.x,
                position.y,
            );
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
}
