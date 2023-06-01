//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_asset::{
    asset::{vxl::VxlFile, Palette}, 
    loader::{PaletteLoader}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<VxlFile>()
        .add_asset::<Palette>()
        .add_asset_loader(PaletteLoader)
        // .init_asset_loader::<CustomAssetLoader>()
        .add_systems(Startup, setup)
        // .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// fn print_on_load(mut state: ResMut<State>, custom_assets: ResMut<Assets<CustomAsset>>) {
//     let custom_asset = custom_assets.get(&state.handle);
//     if state.printed || custom_asset.is_none() {
//         return;
//     }

//     info!("Custom asset loaded: {:?}", custom_asset.unwrap());
//     state.printed = true;
// }
