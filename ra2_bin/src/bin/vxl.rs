//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use ra2_asset::{
    asset::{Palette, VxlFile},
    loader::{PaletteLoader, VxlAssetLoader}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<VxlFile>()
        .add_asset::<Palette>()
        .add_asset_loader(PaletteLoader)
        .add_asset_loader(VxlAssetLoader)
        // .init_asset_loader::<CustomAssetLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assert_server: ResMut<AssetServer>
) {
    let vxl = assert_server.load("vxl/1tnk.vxl");
    let palette = assert_server.load("palettes/uniturb.pal");

    commands.insert_resource(CustomRes {
        vxl,
        palette,
        printed: false
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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

fn print_on_load(
    mut state: ResMut<CustomRes>,
    vxl_assets: ResMut<Assets<VxlFile>>,
    palette_assets: ResMut<Assets<Palette>>
) {
    let vxl_asset = vxl_assets.get(&state.vxl);
    let palette_asset = palette_assets.get(&state.palette);
    if state.printed {
        return;
    }
    let (Some(vxl), Some(palette)) = (vxl_asset, palette_asset) else {
        return;
    };
    state.printed = true;
    info!("vxl asset loaded: {:?}", vxl);
    info!("palette asset loaded: {:?}", palette);
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette: Handle<Palette>,
    pub vxl:     Handle<VxlFile>,
    pub printed: bool
}
