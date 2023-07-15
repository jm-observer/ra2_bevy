use crate::{pub_use::*, shp::ShpFile, tile::TileFile, vxl::Section};
use ra2_util::{read_common_from_file, read_value_from_file, ASSETS_INIS_PATH, ASSETS_PATH};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResourceType {
    TilesSnow,
    ///地形：水、各种草地
    TilesTemp,
    TilesUrb,
    BuildGen,
    BuildSnow,
    BuildTemp,
    BuildUrb,
    TheaterSnow,
    ///覆盖物：桥、路灯、树、弹坑
    TheaterTemp,
    TheaterUrb,
    Ui,
    UiAlly,
    UiSov,
    Anims,
    Vxl,
    Cameo,
    Palettes,
    Ini,
    IniArtPatch,
    IniRulesPatch,
    MapList,
    Strings,
    MainMov,
    EvaAlly,
    EvaSov,
    Sounds,
    UiSounds
}
impl ResourceType {
    pub fn to_string(&self) -> String {
        self.into()
    }

    pub fn get_resource_configs(&self) -> ResourceInfo {
        match self {
            ResourceType::TilesSnow => ResourceInfo {
                id:           "tilesSnow".to_string(),
                src:          "tiles.sno.json".to_string(),
                is_directory: true
            },
            ResourceType::TilesTemp => ResourceInfo {
                id:           "tilesTemp".to_string(),
                src:          "tiles.tem.json".to_string(),
                is_directory: false
            },
            ResourceType::TilesUrb => ResourceInfo {
                id:           "tilesUrb".to_string(),
                src:          "tiles.urb.json".to_string(),
                is_directory: true
            },
            ResourceType::BuildGen => ResourceInfo {
                id:           "buildGen".to_string(),
                src:          "build.gen.json".to_string(),
                is_directory: false
            },
            ResourceType::BuildSnow => ResourceInfo {
                id:           "build.sno".to_string(),
                src:          "build.snow.json".to_string(),
                is_directory: true
            },
            ResourceType::BuildTemp => ResourceInfo {
                id:           "build.temp".to_string(),
                src:          "build.temp.json".to_string(),
                is_directory: false
            },
            ResourceType::BuildUrb => ResourceInfo {
                id:           "build.urb".to_string(),
                src:          "build.urb.json".to_string(),
                is_directory: true
            },
            ResourceType::TheaterSnow => ResourceInfo {
                id:           "theater.snow".to_string(),
                src:          "theater.snow.json".to_string(),
                is_directory: false
            },
            ResourceType::TheaterTemp => ResourceInfo {
                id:           "theater.temp".to_string(),
                src:          "theater.temp.json".to_string(),
                is_directory: false
            },
            ResourceType::TheaterUrb => ResourceInfo {
                id:           "theater.urb".to_string(),
                src:          "theater.urb.json".to_string(),
                is_directory: false
            },
            ResourceType::UiAlly => ResourceInfo {
                id:           "uially".to_string(),
                src:          "uially.json".to_string(),
                is_directory: false
            },
            ResourceType::UiSov => ResourceInfo {
                id:           "uisov".to_string(),
                src:          "uisov.json".to_string(),
                is_directory: false
            },
            ResourceType::Anims => ResourceInfo {
                id:           "anims".to_string(),
                src:          "anims.json".to_string(),
                is_directory: false
            },
            ResourceType::Vxl => ResourceInfo {
                id:           "vxl".to_string(),
                src:          "vxl.json".to_string(),
                is_directory: false
            },
            ResourceType::Cameo => ResourceInfo {
                id:           "cameo".to_string(),
                src:          "cameo.json".to_string(),
                is_directory: false
            },
            ResourceType::Palettes => ResourceInfo {
                id:           "palettes".to_string(),
                src:          "palettes.pal".to_string(),
                is_directory: false
            },
            ResourceType::Ini => ResourceInfo {
                id:           "ini".to_string(),
                src:          "ini.json".to_string(),
                is_directory: false
            },
            ResourceType::IniArtPatch => ResourceInfo {
                id:           "iniArtPatch".to_string(),
                src:          "art_patch.ini".to_string(),
                is_directory: false
            },
            ResourceType::IniRulesPatch => ResourceInfo {
                id:           "iniRulesPatch".to_string(),
                src:          "rules_patch.ini".to_string(),
                is_directory: false
            },
            ResourceType::MapList => ResourceInfo {
                id:           "maplist".to_string(),
                src:          "maps.ini".to_string(),
                is_directory: false
            },
            ResourceType::Ui => ResourceInfo {
                id:           "ui".to_string(),
                src:          "ui.json".to_string(),
                is_directory: false
            },
            ResourceType::Strings => ResourceInfo {
                id:           "strings".to_string(),
                src:          "ra2.csf.json".to_string(),
                is_directory: false
            },
            ResourceType::MainMov => ResourceInfo {
                id:           "mainmov".to_string(),
                src:          "ra2ts_l.mp4".to_string(),
                is_directory: false
            },
            ResourceType::EvaAlly => ResourceInfo {
                id:           "evaally".to_string(),
                src:          "eva.ally.json".to_string(),
                is_directory: false
            },
            ResourceType::EvaSov => ResourceInfo {
                id:           "evasov".to_string(),
                src:          "eva.sov.json".to_string(),
                is_directory: false
            },
            ResourceType::Sounds => ResourceInfo {
                id:           "sounds".to_string(),
                src:          "sounds.json".to_string(),
                is_directory: true
            },
            ResourceType::UiSounds => ResourceInfo {
                id:           "uisounds".to_string(),
                src:          "uisounds.json".to_string(),
                is_directory: false
            }
        }
    }

    pub fn load_resource_vxl_info(&self) -> HashMap<String, Section> {
        //todo
        HashMap::with_capacity(50)
        // match self {
        //     ResourceType::Vxl => {}
        //     _ => panic!(),
        // }
        // let ri = self.get_resource_configs();
        // if ri.is_directory {
        //     //todo
        //     panic!("");
        // }
        // let mut path = String::from(ASSETS_PATH);
        // path.push_str(&ri.src);
        // read_common_from_file(path.as_str())
    }

    pub fn load_resource_shp_info(&self) -> Result<HashMap<String, ShpFile>> {
        match self {
            ResourceType::BuildGen
            | ResourceType::BuildTemp
            | ResourceType::BuildSnow
            | ResourceType::Anims
            | ResourceType::BuildUrb => {},
            _ => {
                println!("load_resource_shp_info:panic!{:?}", self);
                panic!()
            }
        }
        let ri = self.get_resource_configs();
        if ri.is_directory {
            //todo
            println!("load_resource_shp_info:panic!{:?}", self);
            panic!("");
        }
        let mut path = String::from(ASSETS_INIS_PATH);
        path.push_str(&ri.src);
        read_common_from_file(path.as_str())
    }

    pub fn load_resource_tem_info(&self) -> Result<HashMap<String, TileFile>> {
        match self {
            ResourceType::TilesSnow
            | ResourceType::TilesTemp
            | ResourceType::TilesUrb
            | ResourceType::TheaterUrb
            | ResourceType::TheaterSnow
            | ResourceType::TheaterTemp => {},
            _ => panic!()
        }
        let ri = self.get_resource_configs();
        if ri.is_directory {
            //todo
            panic!("");
        }
        let mut path = String::from(ASSETS_PATH);
        path.push_str(&ri.src);
        read_common_from_file(path.as_str())
    }
}

impl From<&ResourceType> for String {
    fn from(item: &ResourceType) -> Self {
        match item {
            ResourceType::TilesSnow => "TilesSnow".to_string(),
            ResourceType::TilesTemp => "TilesTemp".to_string(),
            ResourceType::TilesUrb => "TilesUrb".to_string(),
            ResourceType::BuildGen => "BuildGen".to_string(),
            ResourceType::BuildSnow => "BuildSnow".to_string(),
            ResourceType::BuildTemp => "BuildTemp".to_string(),
            ResourceType::BuildUrb => "BuildUrb".to_string(),
            ResourceType::TheaterSnow => "TheaterSnow".to_string(),
            ResourceType::TheaterTemp => "TheaterTemp".to_string(),
            ResourceType::TheaterUrb => "TheaterUrb".to_string(),
            ResourceType::Ui => "Ui".to_string(),
            ResourceType::UiAlly => "UiAlly".to_string(),
            ResourceType::UiSov => "UiSov".to_string(),
            ResourceType::Anims => "Anims".to_string(),
            ResourceType::Vxl => "Vxl".to_string(),
            ResourceType::Cameo => "Cameo".to_string(),
            ResourceType::Palettes => "Palettes".to_string(),
            ResourceType::Ini => "Ini".to_string(),
            ResourceType::IniArtPatch => "IniArtPatch".to_string(),
            ResourceType::IniRulesPatch => "IniRulesPatch".to_string(),
            ResourceType::MapList => "MapList".to_string(),
            ResourceType::Strings => "Strings".to_string(),
            ResourceType::MainMov => "MainMov".to_string(),
            ResourceType::EvaAlly => "EvaAlly".to_string(),
            ResourceType::EvaSov => "EvaSov".to_string(),
            ResourceType::Sounds => "Sounds".to_string(),
            ResourceType::UiSounds => "TilesSnow".to_string()
        }
    }
}

pub fn load_resource_json_info(rt: ResourceType) -> Result<Value> {
    let ri = rt.get_resource_configs();
    if ri.is_directory {
        //todo
        panic!("");
    }
    let mut path = String::from(ASSETS_PATH);
    path.push_str(&ri.src);
    read_value_from_file(path.as_str())
}

// pub fn load_resource_ini_info(rt: ResourceType) -> Ini {
//     let ri = rt.get_resource_configs();
//     if ri.is_directory {
//         //todo
//         panic!("");
//     }
//     let mut config = Ini::new();
//     let mut path = String::from(ASSETS_INIS_PATH);
//     path.push_str(&ri.src);
//     config.load(path.as_str()).unwrap();
//     config
// }

pub struct ResourceInfo {
    id:           String,
    src:          String,
    is_directory: bool
}
