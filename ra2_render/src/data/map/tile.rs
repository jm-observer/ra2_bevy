use anyhow::Result;
use bevy::{
    asset::Handle,
    ecs::component::Component,
    log::{info, warn},
    prelude::Image
};
use num_traits::cast::FromPrimitive;
use ra2_asset::asset::TileTexture;
use ra2_data::{
    ini::IniFile,
    map::MapTile,
    rule::GeneralRules,
    size::WHSize,
    ty::{LandType, TerrainType}
};
use ra2_util::{pad, random_usize};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Formatter},
    sync::Arc
};

#[derive(Component, Clone)]
pub struct MapTileUnitComponent(pub Arc<MapTileUnit>);

pub struct MapTileCollection {
    pub tile_sets:          Arc<TileCollection>,
    pub general_rules:      Arc<GeneralRules>,
    pub r_size:             WHSize,
    pub d_size:             WHSize,
    pub tiles_by_rxy:       HashMap<i32, Arc<MapTileUnit>>,
    pub tiles_by_dxy:       HashMap<i32, Arc<MapTileUnit>>,
    pub tiles:              Vec<MapTileUnitComponent>,
    pub max_tile_height:    i32,
    pub cutoff_tile_height: i32
}

impl MapTileCollection {
    pub fn init(
        map_tiles: &Vec<MapTile>,
        arc_tile_collection: Arc<TileCollection>,
        general_rule: Arc<GeneralRules>
    ) -> Self {
        let mut r_size = WHSize::init();
        let mut a_size = WHSize::init();
        for map in map_tiles {
            r_size.width = r_size.width.max(map.rx);
            r_size.height = r_size.height.max(map.ry);
            a_size.width = a_size.width.max(map.dx);
            a_size.height = a_size.height.max(map.dx);
        }
        r_size.width += 1;
        r_size.height += 1;
        a_size.width += 1;
        a_size.height += 1;
        println!(
            "r_size {:?} a_size {:?} map_tiles.len()={:?}",
            r_size,
            a_size,
            map_tiles.len()
        );
        let mut tiles_by_rxy =
            HashMap::<i32, MapTileUnit>::with_capacity((r_size.width * r_size.height) as usize);
        let mut tiles_by_dxy = HashMap::<i32, Arc<MapTileUnit>>::with_capacity(100);
        let mut tiles_tmp = Vec::<i32>::with_capacity(map_tiles.len());
        let mut cliff = Vec::<(i32, i32)>::with_capacity(30);
        for mt in map_tiles {
            if let Some(tf) = arc_tile_collection.get_tile_image(mt.tile_num, mt.sub_tile) {
                let image = &tf.images[mt.sub_tile];
                let tmp_image = &tf.file.images[mt.sub_tile];

                let (offset_x, offset_y) = if tmp_image.has_extra_data {
                    (
                        // 0.max(tmp_image.x - tmp_image.extra_x),
                        // 0.max(tmp_image.y - tmp_image.extra_y),
                        // debug
                        0.max(tmp_image.x - tmp_image.extra_x) as f32,
                        (0.max(tmp_image.y - tmp_image.extra_y) / 2) as f32
                    )
                } else {
                    (0.0, 0.0)
                };
                let id = format!("{}_{}", mt.rx, mt.ry);
                let terrain_type = TerrainType::from_i32(tmp_image.terrain_type).unwrap();
                let unit = MapTileUnit {
                    dx: mt.dx,
                    dy: mt.dy,
                    rx: mt.rx,
                    ry: mt.ry,
                    offset_x,
                    offset_y,
                    z: mt.z,
                    tile_num: mt.tile_num,
                    sub_tile: mt.sub_tile,
                    terrain_type,
                    land_type: LandType::get_land_type(terrain_type),
                    on_bridge_land_type: None,
                    ramp_type: tmp_image.ramp_type,
                    id,
                    bitmap: image.clone(),
                    block_width: tf.file.block_width as i32,
                    block_heigth: tf.file.block_height as i32
                };
                match terrain_type {
                    TerrainType::Cliff => {
                        cliff.push((unit.rx as i32, unit.ry as i32));
                    },
                    _ => {}
                }
                // tiles.push(unit.clone());
                let index = unit.rx + unit.ry * r_size.width;
                tiles_tmp.push(index);
                tiles_by_rxy.insert(index, unit);
            } else {
                info!(
                    "##########################################################{:?}",
                    mt
                );
            }

            // tiles_by_dxy.insert(
            //     unit.as_ref().borrow().dx + unit.as_ref().borrow().dy *
            // a_size.width,     unit.clone(),
            // );
            //
        }
        Self::compute_land_behind_cliff_tiles_other(
            general_rule.cliff_back_impassability,
            cliff,
            r_size.width,
            r_size.height,
            &mut tiles_by_rxy
        );

        let tiles_by_rxy = tiles_by_rxy
            .into_iter()
            .map(|(key, val)| {
                let val = Arc::new(val);
                // tiles.push(val.clone());
                tiles_by_dxy.insert(val.dx + val.dy * a_size.width, val.clone());
                (key, val)
            })
            .collect::<HashMap<i32, Arc<MapTileUnit>>>();

        let tiles = tiles_tmp
            .iter()
            .map(|x| MapTileUnitComponent(tiles_by_rxy.get(&x).unwrap().clone()))
            .collect::<Vec<MapTileUnitComponent>>();

        let cutoff_tile_height = Self::compute_cutoff_tile_height(a_size, &tiles_by_dxy);
        let max_tile_height = Self::compute_max_tile_height(&tiles);
        let mtc = MapTileCollection {
            tile_sets: arc_tile_collection,
            general_rules: general_rule,
            r_size,
            d_size: a_size,
            tiles_by_rxy,
            tiles_by_dxy,
            tiles,
            max_tile_height,
            cutoff_tile_height
        };
        mtc
    }

    pub fn compute_land_behind_cliff_tiles_other(
        t: i32,
        e: Vec<(i32, i32)>,
        r_size_width: i32,
        r_size_height: i32,
        tiles_by_rxy: &mut HashMap<i32, MapTileUnit>
    ) {
        for (rx, ry) in e {
            for n in 1..=t {
                for s in 1..=t {
                    let e = rx - n;
                    let t = ry - s;
                    if !(e >= r_size_width || t >= r_size_height) {
                        if let Some(tile) = tiles_by_rxy.get_mut(&(e + t * r_size_width)) {
                            tile.land_type = LandType::Rock;
                        }
                    }
                }
            }
        }
    }

    pub fn compute_cutoff_tile_height(
        d_size: WHSize,
        tiles_by_dxy: &HashMap<i32, Arc<MapTileUnit>>
    ) -> i32 {
        let e = d_size.width - 1;
        let t = d_size.height - 1;
        let mut i = 0;
        for n in 1..(e - 3) {
            let coord = tiles_by_dxy.get(&(n + t * d_size.width));
            if coord.is_some() {
                let coord = coord.unwrap().as_ref().z;
                if coord > i {
                    i = coord;
                }
            }
        }
        i as i32
    }

    // pub fn get_all_neighbour_tiles(&self, mtu: &Rc<RefCell<MapTileUnit>>) ->
    // Vec<Arc<MapTileUnit>> {     let mtu = mtu.as_ref().borrow();
    //     let t = mtu.rx;
    //     let i = mtu.ry;
    //     let mut res = Vec::<Option<Arc<MapTileUnit>>>::with_capacity(8);
    //     res.push(self.get_by_map_coords(t + 1, i + 1));
    //     res.push(self.get_by_map_coords(t - 1, i - 1));
    //     res.push(self.get_by_map_coords(t - 1, i + 1));
    //     res.push(self.get_by_map_coords(t + 1, i - 1));
    //     res.push(self.get_by_map_coords(t, i + 1));
    //     res.push(self.get_by_map_coords(t + 1, i));
    //     res.push(self.get_by_map_coords(t - 1, i));
    //     res.push(self.get_by_map_coords(t, i - 1));
    //     res.into_iter()
    //         .filter(|x| x.is_some())
    //         .map(|x| x.unwrap().clone())
    //         .collect::<Vec<Arc<MapTileUnit>>>()
    // }

    // pub fn get_neigh_bour_tile(
    //     &self,
    //     mtu: &MapTileUnit,
    //     t: TileDirection
    // ) -> Option<Arc<MapTileUnit>> {
    //     let i = mtu.rx;
    //     let n = mtu.ry;
    //     match t {
    //         TileDirection::Bottom => self.get_by_map_coords(i + 1, n + 1),
    //         TileDirection::Top => self.get_by_map_coords(i - 1, n - 1),
    //         TileDirection::Left => self.get_by_map_coords(i - 1, n + 1),
    //         TileDirection::Right => self.get_by_map_coords(i + 1, n - 1),
    //         TileDirection::BottomLeft => self.get_by_map_coords(i, n + 1),
    //         TileDirection::BottomRight => self.get_by_map_coords(i + 1, n),
    //         TileDirection::TopLeft => self.get_by_map_coords(i - 1, n),
    //         TileDirection::TopRight => self.get_by_map_coords(i, n - 1)
    //     }
    // }

    // pub fn get_tile_radar_color(&self, e: &Rc<RefCell<MapTileUnit>>) {
    //     // let e = e.as_ref().borrow();
    //     //return this.tileSets.getTileImage(e.tileNum,
    //     // e.subTile).radarLeft.clone().multiplyScalar(.5) self.tile_sets
    //     //     .get_tile_image(e.tile_num, e.sub_tile)
    //     //     .get_radar_left();
    //     panic!("");
    // }

    pub fn get_all(&self) -> &Vec<MapTileUnitComponent> {
        &self.tiles
    }

    pub fn get_max_tile_height() {}

    pub fn compute_max_tile_height(tiles: &Vec<MapTileUnitComponent>) -> i32 {
        let mut max = 0;
        for tmp in tiles.as_slice() {
            max = max.max(tmp.0.as_ref().z)
        }
        max as i32
    }

    pub fn get_by_display_coords(&self, e: i32, t: i32) -> Option<&Arc<MapTileUnit>> {
        if !(e >= self.d_size.width || t >= self.d_size.height) {
            self.tiles_by_dxy.get(&(e + t * self.d_size.width))
        } else {
            None
        }
    }

    pub fn get_by_map_coords(&self, e: i32, t: i32) -> Option<Arc<MapTileUnit>> {
        if !(e >= self.r_size.width || t >= self.r_size.height) {
            if let Some(res) = self.tiles_by_rxy.get(&(e + t * self.r_size.width)) {
                return Some(res.clone());
            }
        }
        None
    }

    pub fn get_map_size(&self) -> &WHSize {
        &self.r_size
    }

    pub fn get_display_size(&self) -> &WHSize {
        &self.d_size
    }
}

pub struct TileCollection {
    theater_ini:   IniFile,
    extension:     String,
    tile_sets:     Vec<TileSet>,
    order_entries: Vec<Arc<TileSetEntry>>
}
impl TileCollection {
    pub fn new(
        theater_ini: IniFile,
        extension: String,
        tile_datas: &HashMap<String, TileTexture>
    ) -> Self {
        info!("开始初始化TileCollection");
        let mut tc = Self {
            theater_ini,
            extension,
            tile_sets: Vec::<TileSet>::with_capacity(20),
            order_entries: Vec::with_capacity(20)
        };
        tc.load_tile_data(tile_datas);
        info!("TileCollection初始化完毕");
        tc
    }

    pub fn get_tile(&self, index: usize) -> &TileSetEntry {
        &self.order_entries[index]
    }

    pub fn get_tile_image(&self, tile_index: usize, index: usize) -> Option<&TileTexture> {
        let tile = self.get_tile(tile_index);
        let res = tile.get_tmp_file_default(index);
        if res.is_none() {
            warn!("{} {} {:?}", tile_index, index, tile);
        }
        res.map(|(x, _index)| x)
    }

    pub fn get_set_num_todo(&self, _index: usize) -> usize {
        0
    }

    pub fn get_tile_num_from_set() {
        //todo
    }

    pub fn get_general_value(&self, key: &str) -> Result<i32> {
        let t = self.theater_ini.get_section("General")?;
        Ok(t.get_number(key) as i32)
    }

    // pub fn load_tile_data(&mut self, tile_datas: &HashMap<String,
    // Arc<TmpFileData>>) {     self.init_tile_sets(tile_datas);
    // }
    ///初始化tile包括动画
    fn load_tile_data(&mut self, tile_datas: &HashMap<String, TileTexture>) {
        let mut a = 0u8;
        let a_char_val = 'a' as u8;
        let z_char_val = 'z' as u8;
        info!("tile_datas.len={:}", tile_datas.len());
        loop {
            let key = format!("TileSet{}", pad(a as i32, 4));
            let tmp = self.theater_ini.get_section_option(key.as_str());
            if let Some(section) = tmp {
                a += 1;
                let set_name = section.get_string("SetName");
                let mut r = TileSet::new(
                    section.get_string("FileName"),
                    set_name.clone(),
                    section.get_number_i32_from_str("TilesInSet") as i32
                );
                let anim = self.theater_ini.get_section_option(set_name.as_str());
                for t in 1..=r.tiles_in_set {
                    //tiles_in_set在map文件中存在为0的情况，这种情况程序是如何处理的，
                    // 记得调试看看。todo println!("t={}", t);
                    let mut o = r.file_name.clone() + pad(t, 2).as_str();
                    let mut s = TileSetEntry::new(o.clone(), t - 1);
                    let mut n = a_char_val - 1u8;
                    while n < z_char_val {
                        if n >= a_char_val && "Bridges" == r.set_name {
                            // n += 1;
                            // ?
                            break;
                        }
                        if n >= a_char_val {
                            o.push(n as char);
                        }
                        o.push_str(self.extension.as_str());

                        //不能用tile_datas.take(&o)，因为存在重复获取的情况
                        let l = tile_datas.get(&o);
                        if let Some(l) = l {
                            s.add_file(l.clone());
                            n += 1;
                        } else {
                            if s.files.len() == 0 {
                                // 极有可能为shp文件，在theater.tem.json文件中
                                // 已确认的shp文件：mslop*.tem
                                // println!("order_entries.len()={} tile_name={}
                                // set_name={:?} file_name:{}",
                                // self.order_entries.len(), o.as_str(),
                                // r.set_name, r.file_name);
                            }
                            break;
                        }
                    }
                    if s.files.len() == 0 {
                        // 极有可能为shp文件，在theater.tem.json文件中
                        // 已确认的shp文件：mslop*.tem
                        // println!("order_entries.len()={} set_name={:?}
                        // file_name:{}"     , self.
                        // order_entries.len(), r.set_name, r.file_name);
                    }

                    //初始化动画
                    if let Some(anim) = anim {
                        let tmp_t = format!("Tile{}", pad(t, 2));
                        let a = format!("{}Anim", tmp_t);
                        let o = anim.get_string_option(a.as_str());
                        if let Some(o) = o {
                            let l = TileSetAnim::new(
                                o,
                                anim.get_number_i32(format!("{}AttachesTo", tmp_t).as_str())
                                    as usize,
                                anim.get_number_i32(format!("{}XOffset", tmp_t).as_str()),
                                anim.get_number_i32(format!("{}YOffset", tmp_t).as_str()),
                                anim.get_number_i32(format!("{}ZAdjust", tmp_t).as_str())
                            );
                            s.set_animation(l);
                        } else {
                            println!("warn miss anim");
                        }
                    }
                    let arc_s = Arc::new(s);
                    self.order_entries.push(arc_s.clone());
                    r.entries.push(arc_s);
                }
                if r.tiles_in_set != r.entries.len() as i32 {
                    panic!()
                }
                self.tile_sets.push(r);
            } else {
                warn!("theater_ini could not contain: {}", key);
                break;
            }
        }
    }

    // 已合并至init_tile_sets
    // pub fn init_animations(&self) {
    // }
    pub fn is_lat(&self, e: i32) -> Result<bool> {
        Ok(if e == self.get_general_value("RoughTile")? {
            true
        } else if e == self.get_general_value("SandTile")? {
            true
        } else if e == self.get_general_value("GreenTile")? {
            true
        } else if e == self.get_general_value("PaveTile")? {
            true
        } else {
            false
        })
    }

    pub fn is_clat(&self, _e: u32) -> bool {
        //todo
        false
    }
}

// 某种地形的集合，如clat01.sno clat02.sno cfii01.tem
// cfiow02.tem等，之间为不同的地形
#[derive(Debug)]
pub struct TileSet {
    file_name:    String,
    set_name:     String,
    tiles_in_set: i32,
    entries:      Vec<Arc<TileSetEntry>>
}

impl TileSet {
    pub fn new(
        file_name: String,
        set_name: String,
        tiles_in_set: i32 // entries: Vec<String>,
    ) -> Self {
        TileSet {
            file_name,
            set_name,
            tiles_in_set,
            entries: Vec::with_capacity(10)
        }
    }
}

// impl TileTexture {
//     pub fn from(tile_asset: &TileAsset, pal: &Palette) -> Self {}
// }
///某种地形的集合，如clat01.sno clat01a.sno clat01b.sno
/// 等等为一个TileSetEntry，为同一个地形的不同选择，渲染时随机选择
pub struct TileSetEntry {
    pub name:  String,
    // owner: &'a TileSet<'a>,
    index:     i32,
    // files: Vec<Arc<TmpFileData>>,
    files:     Vec<TileTexture>,
    animation: Option<TileSetAnim>
}

impl Debug for TileSetEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("index={}, files.len={}", self.index, self.files.len()).as_str())
    }
}
impl TileSetEntry {
    //owner: &'a TileSet,
    pub fn new(name: String, index: i32) -> Self {
        TileSetEntry {
            name,
            index,
            files: Vec::with_capacity(20),
            animation: None
        }
    }

    pub fn add_file(
        &mut self,
        file: TileTexture /* assets: &mut Assets<Image>,
                           * pal: Arc<Palette>, */
    ) {
        // let mut images: Vec<Handle<Texture>> = Vec::with_capacity(file.images.len());
        // // let mut has_damaged_data: Vec<bool> =
        // Vec::with_capacity(file.images.len()); for bitmap in &file.images {
        //     let mut sprite_bitmap = Bitmap::rgba_bitmap(bitmap.width, bitmap.height);
        //     sprite_bitmap.draw_indexed_image(bitmap.as_ref(), 0, 0);
        //     sprite_bitmap.update_from_palette(&pal);
        //     images.push(assets.add(sprite_bitmap.into()));
        // }
        self.files.push(file);
    }

    pub fn set_animation(&mut self, animation: TileSetAnim) {
        self.animation = Some(animation)
    }

    pub fn get_animation(&self) -> Option<&TileSetAnim> {
        return self.animation.as_ref();
    }

    pub fn get_tmp_file_default(&self, index: usize) -> Option<(&TileTexture, usize)> {
        self.get_tmp_file(index, false)
    }

    pub fn get_tmp_file(&self, _index: usize, _t: bool) -> Option<(&TileTexture, usize)> {
        if self.files.len() <= 0 {
            println!("tmp_file.len()==0");
            return None;
        }
        let n = random_usize(0, self.files.len());
        let file = &self.files[n];
        Some((file, n))
        // Some((file.clone(), n))
        //感觉没有必要这么绕
        // let index = index.min(file.images.len() - 1);
        // if file.file.images[index].has_damaged_data {
        //     let min = (if t { 1 } else { 0 }).min(self.files.len() - 1);
        //     Some((&self.files[min], min))
        // } else {
        //     Some((file, n))
        // }
    }
}

#[derive(Debug)]
pub struct TileSetAnim {
    pub name:     String,
    pub sub_tile: usize,
    pub offset_x: i32,
    pub offset_y: i32,
    pub adjust_z: i32
}
impl TileSetAnim {
    pub fn new(name: String, sub_tile: usize, offset_x: i32, offset_y: i32, adjust_z: i32) -> Self {
        TileSetAnim {
            name,
            sub_tile,
            offset_x,
            offset_y,
            adjust_z
        }
    }
}

#[derive(Debug, Component)]
pub struct MapTileUnit {
    /// xy方向的第几个tile
    pub dx:                  i32,
    pub dy:                  i32,
    /// 暂时未知
    pub rx:                  i32,
    pub ry:                  i32,
    /// xy方向偏移像素
    pub offset_x:            f32,
    pub offset_y:            f32,
    pub z:                   i32,
    pub tile_num:            usize,
    pub sub_tile:            usize,
    // map_tile: MapTile,
    pub terrain_type:        TerrainType,
    pub land_type:           LandType,
    pub on_bridge_land_type: Option<LandType>,
    pub ramp_type:           i32,
    pub id:                  String,
    pub bitmap:              Handle<Image>,
    pub block_width:         i32,
    pub block_heigth:        i32
}
