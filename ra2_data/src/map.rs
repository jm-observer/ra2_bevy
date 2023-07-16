mod aircraft;
mod infantry;
mod smudge;
mod tag;
mod terrain;
mod trigger;
mod vehicle;

pub use aircraft::*;
pub use infantry::*;
pub use smudge::*;
pub use tag::*;
pub use terrain::*;
pub use trigger::*;
pub use vehicle::*;

use crate::{
    extend_json::JsonExtend,
    ini::{IniFile, IniSection},
    lighting::Lighting,
    map_object::MapObject,
    overlay::Overlay,
    pub_use::*,
    size::RaSize,
    structure::Structure,
    theater::TheaterType
};
use ra2_util::as_number;
use serde_json::Value;
use std::sync::Arc;

pub struct MapFile {
    // pub rule: Arc<Rule>,
    pub full_size:    RaSize,
    pub size:         RaSize,
    pub theater:      TheaterType,
    pub ini_file:     IniFile,
    pub tiles:        Vec<MapTile>,
    pub lighting:     Lighting,
    pub ion_lighting: Lighting,
    pub tags:         Vec<Arc<Tag>>,
    pub triggers:     Vec<TriggerUnit>,
    pub waypoints:    Vec<Waypoint>,
    pub structures:   Vec<Structure>,
    pub terrains:     Vec<Arc<MapObject>>,
    pub overlays:     Vec<Overlay>,
    pub smudges:      Vec<Arc<MapObject>>,
    pub infantries:   Vec<Arc<MapObject>>,
    pub vehicles:     Vec<Arc<MapObject>>, /*  */
    pub aircrafts:    Vec<Arc<MapObject>>
}

impl MapFile {
    pub fn init(vals: &Value) -> Result<Self> {
        let _ini_data = vals.get("ini_data").unwrap();
        // println!("ini_data={:?}", ini_data);
        let mut ini_file = IniFile::new(
            "ini_data".to_string(),
            vals.get("ini_data").unwrap().clone(),
            None
        )?;
        let full_size_val = vals.get("full_size").unwrap();
        let full_size = RaSize {
            x:      full_size_val.get_i32("x"),
            y:      full_size_val.get_i32("y"),
            width:  full_size_val.get_i32("width"),
            height: full_size_val.get_i32("height")
        };
        let local_size_val = vals.get("local_size").unwrap();
        let size = RaSize {
            x:      local_size_val.get_i32("x"),
            y:      local_size_val.get_i32("y"),
            width:  local_size_val.get_i32("width"),
            height: local_size_val.get_i32("height")
        };
        // let full_size = MapSize::init(vals.get("full_size").unwrap());
        // let local_size = MapSize::init(vals.get("local_size").unwrap());

        let theater = TheaterType::get_theater_type_by_str(vals.get_str("theater"));
        let ini_file_tmp = &mut ini_file;
        let tags_section = ini_file_tmp.get_or_create_section("tags");
        let tags = TagsReader::read(tags_section);
        let triggers = ini_file_tmp.get_or_create_section("triggers");
        let events = ini_file_tmp.get_or_create_section("events");
        let actions = ini_file_tmp.get_or_create_section("actions");
        let structures = ini_file_tmp.get_or_create_section("structures");
        let terrain = ini_file_tmp.get_or_create_section("terrain");
        let lighting = ini_file_tmp.get_or_create_section("lighting");
        let trigger = TriggerReader::read(triggers, events, actions, &tags)?;
        // TriggerReader::read()
        let mut map_file = MapFile {
            // rule,
            full_size,
            size,
            theater,
            ini_file,
            tiles: Vec::<MapTile>::with_capacity(500),
            lighting: Lighting::init(),
            ion_lighting: Lighting::init(),
            tags,
            triggers: trigger,
            waypoints: Vec::with_capacity(20),
            structures: Vec::with_capacity(50),
            terrains: Vec::with_capacity(50),
            overlays: Vec::with_capacity(50),
            smudges: Vec::with_capacity(50),
            infantries: Vec::with_capacity(50),
            vehicles: Vec::with_capacity(50),
            aircrafts: Vec::with_capacity(50)
        };
        map_file.load_tiles(vals.get_array("tiles"));
        map_file.load_waypoints();
        map_file.load_structures(structures)?;
        map_file.load_vehicles(vals.get_array("units"));
        map_file.load_infantries()?;
        map_file.load_aircrafts(vals.get_array("aircrafts"));
        map_file.load_terrains(terrain)?;
        map_file.load_overlays(vals.get_array("overlays"));
        map_file.load_smudges(vals.get_array("smudges"));
        map_file.lighting.read_default(lighting.clone());
        map_file.ion_lighting.read(lighting, "ion");
        map_file.ion_lighting.force_tint = true;
        println!("MapFile加载完毕");
        Ok(map_file)
    }

    ///斑点 光点 污点 黑灰
    fn load_smudges(&mut self, vals: Vec<Value>) {
        for val in vals {
            // let val = val.as_object().unwrap();
            self.smudges.push(Arc::new(Smudge::new(
                val.get_string("name"),
                val.get_i32("rx"),
                val.get_i32("ry")
            )));
        }
    }

    ///覆盖图；镶边；包镶物
    fn load_overlays(&mut self, vals: Vec<Value>) {
        for val in vals {
            let id = val.get_i32("id");
            self.overlays.push(Overlay::new(
                // self.rule.overlay_types.get(&id).unwrap().clone(),
                val.get_i32("rx"),
                val.get_i32("ry"),
                id,
                val.get_i32("value") as u32
            ));
        }
    }

    ///地势 地形 地域 地面
    fn load_terrains(&mut self, sec: Arc<IniSection>) -> Result<()> {
        for (key, val) in &sec.entries {
            let e = as_number(key)? as i32;
            self.terrains.push(Arc::new(Terrain::new(
                val.as_str().unwrap().to_string(),
                e % 1000,
                e / 1000
            )));
        }
        Ok(())
    }

    ///飞机；直升机；航空器
    fn load_aircrafts(&mut self, vals: Vec<Value>) {
        for val in vals {
            // let val = val.as_object().unwrap();
            self.aircrafts.push(Arc::new(Aircraft::new(
                val.get_string("name"),
                val.get_i32("rx"),
                val.get_i32("ry"),
                val.get_string("owner"),
                val.get_i32("health"),
                val.get_i32("direction")
            )));
        }
    }

    ///步兵；步兵团
    fn load_infantries(&mut self) -> Result<()> {
        let sec = self.get_or_create_section("Infantry");
        for (_, val) in &sec.entries {
            let val: Vec<&str> = val
                .as_str()
                .ok_or(anyhow!("not a str"))?
                .split(",")
                .collect();
            if val.len() <= 8 {
                continue;
            }
            let moi = Infantry::new(
                val[1].to_string(),
                as_number(val[3])? as i32,
                as_number(val[4])? as i32,
                as_number(val[2])? as i32,
                as_number(val[5])? as i32,
                as_number(val[7])? as i32,
                "1" == val[11],
                val[0].to_string()
            );
            self.infantries.push(Arc::new(moi));
        }
        Ok(())
    }

    ///交通工具 架次 载具
    fn load_vehicles(&mut self, vals: Vec<Value>) {
        for val in vals {
            // let val = val.as_object().unwrap();
            self.vehicles.push(Arc::new(Vehicle::new(
                val.get_string("name"),
                val.get_i32("rx"),
                val.get_i32("ry"),
                val.get_string("owner"),
                val.get_i32("health"),
                val.get_i32("direction"),
                val.get_bool("on_bridge")
            )));
        }
    }

    fn load_structures(&mut self, sec: Arc<IniSection>) -> Result<()> {
        for (_, val) in &sec.entries {
            let val: Vec<&str> = val.as_str().unwrap().split(",").collect();
            if val.len() <= 15 {
                continue;
            }
            let moi = Structure::new(
                val[1].to_string(),
                as_number(val[3])? as i32,
                as_number(val[4])? as i32,
                &val[0].to_string(),
                as_number(val[2])? as i32
            );
            self.structures.push(moi);
        }
        Ok(())
    }

    fn load_waypoints(&mut self) {
        //todo
    }

    fn load_tiles(&mut self, vals: Vec<Value>) {
        for tmp in vals {
            self.tiles.push(MapTile::init(tmp.as_str().unwrap()));
        }
    }

    fn get_or_create_section(&mut self, key: &str) -> Arc<IniSection> {
        self.ini_file.get_or_create_section(key)
    }
}

#[derive(Debug)]
pub struct MapTile {
    /// dxy 定义地块的位置，dx单位长度ISO_TILE_SIZE，dy单位高度ISO_TILE_HEIGHT
    /// int dx = rx - ry + FullSize.Width - 1;
    /// int dy = rx + ry - FullSize.Width - 1;
    /// 上面是一个线性变换 旋转45度、拉长、平移 作者：HDTT咸鱼 https://www.bilibili.com/read/cv4077879 出处：bilibili
    pub dx:       i32,
    pub dy:       i32,
    /// rxy 具体待理清
    pub rx:       i32,
    pub ry:       i32,
    pub z:        i32,
    pub tile_num: usize,
    pub sub_tile: usize
}
impl MapTile {
    pub fn init(vals: &str) -> Self {
        // println!("{}", vals);
        let slices: Vec<&str> = vals.split(',').collect();
        let mt = MapTile {
            dx:       slices[0].parse::<i32>().unwrap(),
            dy:       slices[1].parse::<i32>().unwrap(),
            rx:       slices[2].parse::<i32>().unwrap(),
            ry:       slices[3].parse::<i32>().unwrap(),
            z:        slices[4].parse::<i32>().unwrap(),
            tile_num: 0i32.max(slices[5].parse::<i32>().unwrap()) as usize,
            sub_tile: slices[6].parse::<usize>().unwrap()
        };
        // if mt.tile_num == 314 {
        //     println!("{:?}", mt);
        // }
        mt
    }
}

pub struct Waypoint {
    pub number: i32,
    pub rx:     i32,
    pub ry:     i32
}

pub struct MapSizeSize {
    pub width:    i32,
    pub height:   i32,
    pub is_empty: bool
}
pub struct MapSizeLocation {
    pub x:        i32,
    pub y:        i32,
    pub is_empty: bool
}
pub struct MapSize {
    bottom:     i32,
    pub height: i32,
    is_empty:   bool,
    left:       i32,
    location:   MapSizeLocation,
    right:      i32,
    size:       MapSizeSize,
    top:        i32,
    pub width:  i32,
    x:          i32,
    y:          i32
}

impl MapSize {
    pub fn init(vals: &Value) -> Self {
        let mut location_val = vals.get("location").unwrap();
        let location = MapSizeLocation {
            x:        location_val.get_i32("x"),
            y:        location_val.get_i32("y"),
            is_empty: location_val.get_bool("is_empty")
        };
        location_val = vals.get("size").unwrap();
        let size = MapSizeSize {
            height:   location_val.get_i32("height"),
            width:    location_val.get_i32("width"),
            is_empty: location_val.get_bool("is_empty")
        };
        MapSize {
            bottom: vals.get_i32("bottom"),
            height: vals.get_i32("height"),
            is_empty: vals.get_bool("is_empty"),
            left: vals.get_i32("left"),
            location,
            right: vals.get_i32("right"),
            size,
            top: vals.get_i32("top"),
            width: vals.get_i32("width"),
            x: vals.get_i32("x"),
            y: vals.get_i32("y")
        }
    }
}
