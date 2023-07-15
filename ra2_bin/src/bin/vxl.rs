//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    prelude::*,
    render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology}
};
use ra2_asset::{
    asset::VxlFile,
    loader::{PaletteLoader, VxlAssetLoader}
};
use ra2_data::color::{Palette, RaColor};

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
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    assert_server: ResMut<AssetServer>
) {
    // let vxl = assert_server.load("vxl/1tnk.vxl");
    // let vxl = assert_server.load("vxl/1tnkbarl.vxl");

    let vxl = assert_server.load("vxl/1tnk.vxl");

    let palette = assert_server.load("palettes/uniturb.pal");

    commands.insert_resource(CustomRes {
        vxl,
        palette,
        printed: false
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
    mut commands: Commands,
    mut state: ResMut<CustomRes>,
    vxl_assets: ResMut<Assets<VxlFile>>,
    mut palette_assets: ResMut<Assets<Palette>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>
) {
    let vxl_asset = vxl_assets.get(&state.vxl);
    let palette_asset = palette_assets.get_mut(&state.palette);
    if state.printed {
        return;
    }
    let (Some(vxl), Some(palette)) = (vxl_asset, palette_asset) else {
        return;
    };
    //     "Green": "104,241,195",
    let player_color = RaColor::new(104, 241, 195);
    palette.remap(&player_color);
    state.printed = true;
    info!("vxl asset loaded: {:?}", vxl);
    info!("palette asset loaded: {:?}", palette);

    for section in &vxl.sections {
        info!("scale {:?}", section.get_scale());
        let normal = section.get_normals();

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();

        let mut render_mesh = Mesh::new(PrimitiveTopology::PointList);

        for vxl in section.get_all_voxels() {
            let color = palette.colors.get(vxl.color_index).unwrap();
            colors.push([color.r as f32, color.g as f32, color.b as f32, 1.0]);
            positions.push([vxl.x, vxl.y, vxl.z]);
            normals.push(normal.get(vxl.normal_index).unwrap().clone());
        }
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Sint32x3(positions)
        );

        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals)
        );
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(colors)
        );

        let handle = meshes.add(render_mesh);

        commands.spawn(PbrBundle {
            transform: Transform::from_scale((0.01, 0.01, 0.01).into()),
            // transform: Transform::from_scale(section.get_scale()),
            mesh: handle,
            // mesh: assets.load("2x2x2.vox"),
            material: stdmats.add(Color::rgb(0.3, 0.3, 0.3).into()),
            ..Default::default()
        });
    }
}

#[derive(Resource)]
pub struct CustomRes {
    pub palette: Handle<Palette>,
    pub vxl:     Handle<VxlFile>,
    pub printed: bool
}
