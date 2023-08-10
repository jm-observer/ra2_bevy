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
    mut command: Commands
) {
    println!("开始渲染地图");
    let tiles = Arc::new(MapTileCollection::init(
        &mr.res.tiles,
        tcr.get_tile_collection(),
        general.0.clone()
    ));
    create_map_tile_sprites(&mut command, tiles);
}

pub fn create_map_tile_sprites(command: &mut Commands, tiles: Arc<MapTileCollection>) {
    println!(
        "d_size.width={} d_size.height={}",
        tiles.d_size.width, tiles.d_size.height
    );
    println!("draw sprint's num : {}", tiles.tiles.len());
    for (_index, arc_map_tile_units) in tiles.tiles.iter().enumerate() {
        let tile = &arc_map_tile_units.0;
        let i = tile.tile_num;
        let n = tiles.tile_sets.get_tile(i);

        // Point offset = new Point(tile.Dx * tmp.BlockWidth / 2, (tile.Dy - tile.Z) *
        // tmp.BlockHeight / 2);

        let x_c = (tile.dx * tile.block_width) as f32 / 2.0;
        let y_c = ((tile.dy - tile.z) * tile.block_heigth) as f32 / 2.0;
        if n.name.starts_with("Cliff32") {
            println!(
                "Drawing TMP file {0} (subtile {1}) at ({2},{3})",
                n.name, tile.tile_num, x_c, y_c
            );
        }

        command
            .spawn(init_sprite(
                tile.bitmap.clone(),
                x_c,        // x + tmp.0,
                y_c * -1.0, // y + tmp.1,
                0.0
            ))
            .insert(TileComponent)
            .insert(arc_map_tile_units.clone());
        if let Some(_anim) = n.get_animation() {}
    }
}

fn init_sprite(bitmap: Handle<Image>, x: f32, y: f32, z: f32) -> SpriteBundle {
    // info!("{} {} {}", x, y, z);
    SpriteBundle {
        transform: Transform::from_xyz(x, y, z),
        texture: bitmap,
        ..Default::default()
    }
}
