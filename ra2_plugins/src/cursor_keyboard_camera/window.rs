use crate::cursor_keyboard_camera::CursorPositionChangeEvent;
use bevy::{prelude::*, window::*};

#[derive(Component)]
pub struct ResolutionText;

pub fn init(commands: &mut Commands) {
    let window_text = "Window:\n".to_string();
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
                bottom: Val::Px(40.0),
                left: Val::Px(10.0),
                ..default()
            })
        )
        .insert(ResolutionText);
}
pub fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut events: EventWriter<CursorPositionChangeEvent>
) {
    if resize_reader.len() == 0 {
        return;
    }
    let mut text = q.single_mut();
    let window = windows.single();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.sections[0].value = format!(
            "Window: {:.1} {:.1} {:.1} {:.1}",
            e.width,
            e.height,
            window.physical_width(),
            window.physical_height()
        );
    }
    events.send(CursorPositionChangeEvent)
}

pub fn on_create_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut _resize_reader: EventReader<WindowCreated>,
    windows: Query<&Window>
) {
    let mut text = q.single_mut();
    let window = windows.single();
    text.sections[0].value = format!(
        "Window: {:.1} {:.1}",
        window.resolution.width(),
        window.resolution.height()
    );
    // }
}
