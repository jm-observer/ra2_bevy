use crate::{
    component::TileComponent,
    data::map::MapTileCollection,
    resource::map::{GeneralRuleRes, MapRes, TileCollectionRes}
};
use bevy::prelude::*;
use std::sync::Arc;

/// 渲染地图
pub fn playtime_enter_map_draw(
    // tcr: Option<Res<TileCollectionRes>>, rr: Option<Res<RuleRes>>, mr: Option<Res<MapRes>>,
    tcr: Res<TileCollectionRes>,
    general: Res<GeneralRuleRes>,
    mr: Res<MapRes>,
    mut command: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    println!("开始渲染地图");
    let tiles = Arc::new(MapTileCollection::init(
        &mr.res.tiles,
        tcr.get_tile_collection(),
        general.0.clone()
    ));
    create_map_tile_sprites(&mut textures, &mut materials, &mut command, tiles);
}

pub fn create_map_tile_sprites(
    textures: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    command: &mut Commands,
    tiles: Arc<MapTileCollection>
) {
    println!(
        "d_size.width={} d_size.height={}",
        tiles.d_size.width, tiles.d_size.height
    );
    println!("draw sprint's num : {}", tiles.tiles.len());
    for (_index, arc_map_tile_units) in tiles.tiles.iter().enumerate() {
        let map_tile_units = &arc_map_tile_units.0;
        let i = map_tile_units.tile_num;
        let n = tiles.tile_sets.get_tile(i);

        let x_c = (map_tile_units.dx * map_tile_units.block_width) as f32 / 2.0;
        let y_c =
            ((map_tile_units.dy - map_tile_units.z) * map_tile_units.block_heigth) as f32 / 2.0;
        command
            .spawn(init_sprinte(
                map_tile_units.bitmap.clone(),
                x_c + map_tile_units.offset_x,        // x + tmp.0,
                y_c * -1.0 + map_tile_units.offset_y, // y + tmp.1,
                0.0,
                textures,
                materials
            ))
            .insert(TileComponent)
            .insert(arc_map_tile_units.clone());
        if let Some(_anim) = n.get_animation() {}
    }
}

fn init_sprinte(
    bitmap: Handle<Image>,
    x: f32,
    y: f32,
    z: f32,
    _textures: &mut ResMut<Assets<Image>>,
    _materials: &mut ResMut<Assets<ColorMaterial>>
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            // 没效果？
            custom_size: Some(Vec2::new(0.5f32, 0.5f32)),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, z),
        texture: bitmap,
        ..Default::default()
    }
}
