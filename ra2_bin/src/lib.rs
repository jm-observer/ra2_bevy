use bevy::{
    app::App,
    asset::AddAsset,
    ecs::component::Component,
    input::Input,
    log::info,
    math::Vec3,
    prelude::{
        CursorMoved, EventReader, KeyCode, NextState, Query, Res, ResMut, Time, Transform, With
    }
};
use ra2_asset::{
    asset::{IniAsset, MapAsset, ShpAsset, TileAsset},
    loader::{IniFileAssetLoader, MapLoader, PaletteLoader, ShpLoader, TilesAssetLoader},
    DebugGameState
};
use ra2_data::color::Palette;
use ra2_plugins::cursor_and_camera::Camera;

pub fn add_assets_and_loaders(app: &mut App) {
    app.add_asset::<MapAsset>()
        .add_asset::<Palette>()
        .add_asset::<TileAsset>()
        .add_asset::<IniAsset>()
        .add_asset::<ShpAsset>()
        .add_asset_loader(PaletteLoader)
        .add_asset_loader(MapLoader)
        .add_asset_loader(TilesAssetLoader)
        .add_asset_loader(IniFileAssetLoader)
        .add_asset_loader(ShpLoader)
        .add_state::<DebugGameState>();
}

#[derive(Component)]
pub struct Cursor;

pub fn listen_keyboard(
    time: Res<Time>,
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
            transform.translation += direction.normalize() * 100.0 * time.delta_seconds();
        }
    }
}
