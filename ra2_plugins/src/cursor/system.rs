use super::{component::*, res::*, res_loader::*};
use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseMotion, ButtonState},
    prelude::*
};
use ra2_asset::{asset::ShpAsset, component::CameraComponent};
use ra2_data::color::Palette;
use std::{
    ops::{Add, Sub},
    time::Duration
};

/// Prepare/Loading加载初始化，加入初始化强校验中
/// PlayTime.enter渲染
pub fn cursor_status_init(
    asset_server: Res<AssetServer>,
    pal_assets: Res<Assets<Palette>>,
    shp_assets: Res<Assets<ShpAsset>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut mouse: ResMut<CursorShpStatus>
) {
    info!("cursor_status_init");
    mouse.init(
        &asset_server,
        &pal_assets,
        &shp_assets,
        &mut textures,
        &mut commands,
        &mut texture_atlases
    );
}
pub fn cursor_status_new(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut checks: ResMut<CheckInitedReses>
) {
    let mouse_stats = CursorShpStatus::new_status(&asset_server);
    checks.add_res(mouse_stats.get_init_status(), "CursorShpStatus");
    commands.insert_resource(mouse_stats);
}

pub fn cursor_draw(mouse_res: Res<CursorShpRes>, mut commands: Commands) {
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: mouse_res.atlas_handle.clone(),
            ..Default::default()
        })
        .insert(Cursor::default());
}

pub fn mouse_events_system(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut q: ParamSet<(
        Query<(&mut Transform, &mut Cursor), With<Cursor>>,
        Query<&Transform, With<CameraComponent>>
    )>,
    cursor_bound: Res<CoordsBound>
) {
    // println!("mouse_events_system");
    // let (mut cursor_tf, mut camera_tf) = q.
    let camera_translation = q.p1().single().translation;
    let mut binding = q.p0();
    let (mut cursor_translation, mut cursor) = binding.single_mut();
    let cursor_translation = &mut cursor_translation.translation;
    let cursor_translation_copy: Vec3 = cursor_translation.clone();
    // let mut is_move = false;
    for event in mouse_motion_events.iter() {
        let MouseMotion { delta } = event;
        cursor_translation.x += delta.x;
        cursor_translation.y -= delta.y;
        // is_move = true;
    }
    /// 光标在边框时，需要自动移动地图
    cursor_bound.auto_move_cursor(cursor_translation, &camera_translation);
    let res = cursor_translation_copy.sub(*cursor_translation);
    if res.x != 0.0 || res.y != 00.0 {
        cursor.0 = true;
        cursor.1 = Duration::new(0, 0)
    } else {
        cursor.0 = false;
        cursor.1 += time.delta();
    }
}
pub fn camera_move_system(
    mut q: ParamSet<(
        Query<&Transform, With<Cursor>>,
        Query<&mut Transform, With<CameraComponent>>
    )>,
    cursor_bound: Res<CoordsBound>
) {
    // println!("camera_move_system");
    let cursor_translation = q.p0().single().translation;
    let mut p0 = q.p1();
    let mut bind = p0.single_mut();
    cursor_bound.adjust_camera_translation(&mut bind.translation, &cursor_translation);
}

/// 鼠标动画测试：每1s更换精灵
// pub fn animate_mouse(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
// ) {
// for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
//     timer.tick(time.delta());
//     if timer.finished() {
//         let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//         sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
//     }
// }
// }

/// This system prints out all keyboard events as they come in
pub fn keyboard_event_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut q: ParamSet<(
        Query<&mut Transform, With<Cursor>>,
        Query<&Transform, With<CameraComponent>>
    )>,
    cursor_bound: Res<CoordsBound>
) {
    let camera_translation = q.p1().single().translation;
    let mut binding = q.p0();
    let cursor_translation = &mut binding.single_mut().translation;
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            match event.state {
                ButtonState::Pressed => match key_code {
                    KeyCode::Right => cursor_translation.x += 20.0,
                    KeyCode::Left => cursor_translation.x -= 20.0,
                    KeyCode::Up => cursor_translation.y += 20.0,
                    KeyCode::Down => cursor_translation.y -= 20.0,
                    _ => {}
                },
                _ => {}
            }
        }
        /// 光标在边框时，需要自动移动地图
        cursor_bound.auto_move_cursor(cursor_translation, &camera_translation);
    }
}
