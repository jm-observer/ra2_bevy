use bevy::prelude::*;

#[derive(Default)]
pub struct CameraPositionChangeEvent(pub f32, pub f32, pub f32);

#[derive(Component)]
pub struct Camera;
#[derive(Component)]
pub struct CameraPositionText;

pub fn update_camera_position(
    mut events: EventReader<CameraPositionChangeEvent>,
    mut camera_position: Query<&mut Transform, With<Camera>>,
    mut camera_position_text: Query<&mut Text, With<CameraPositionText>>
) {
    if events.len() == 0 {
        return;
    }
    let mut camera_position = camera_position.single_mut();
    for event in events.iter() {
        camera_position.translation.x = camera_position.translation.x + event.0;
        camera_position.translation.y = camera_position.translation.y + event.1;
        camera_position.translation.z = camera_position.translation.z + event.2;
    }

    let mut text = camera_position_text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!(
        "Camera Positon x: {} y: {} z: {}",
        camera_position.translation.x, camera_position.translation.y, camera_position.translation.z
    );
}
