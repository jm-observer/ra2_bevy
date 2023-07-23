use bevy::{asset::LoadState, prelude::*};
use ra2_asset::{
    asset::{IniAsset, ShpAsset, TileAsset},
    loader::{IniFileAssetLoader, PaletteLoader, TilesAssetLoader},
    DebugGameState
};
use ra2_bin::add_assets_and_loaders;
use ra2_data::color::Palette;
use ra2_plugins::cursor::{init_cursor, CursorShp};
use std::env;

fn main() {
    env::set_var("BEVY_ASSET_ROOT", "D:\\git");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    add_assets_and_loaders(&mut app);
    app.add_systems(Startup, setup)
        .add_systems(Update, listen_keyboard)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}
fn listen_keyboard(
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
            info!("{}", direction);
            transform.translation += direction.normalize() * 100.0 * time.delta_seconds();
        }
    }
}

#[derive(Component)]
struct Camera;
