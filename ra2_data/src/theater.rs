use crate::resource::ResourceType;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TheaterType {
    Temperate,
    Snow,
    Urban
}
impl Default for TheaterType {
    fn default() -> Self {
        TheaterType::Temperate
    }
}

impl<'a> From<TheaterType> for &'a str {
    fn from(item: TheaterType) -> &'a str {
        match item {
            TheaterType::Snow => "snow",
            TheaterType::Temperate => "temperate",
            TheaterType::Urban => "urban"
        }
    }
}

impl From<TheaterType> for String {
    fn from(item: TheaterType) -> String {
        match item {
            TheaterType::Snow => "snow".to_string(),
            TheaterType::Temperate => "temperate".to_string(),
            TheaterType::Urban => "urban".to_string()
        }
    }
}
// impl std::cmp::Eq for TheaterType {}

// impl Hash for TheaterType {}

impl TheaterType {
    pub fn get_all_theater() -> Vec<TheaterType> {
        [
            TheaterType::Temperate,
            TheaterType::Snow,
            TheaterType::Urban
        ]
        .to_vec()
    }

    // pub fn theater_specific_resources(&self) -> Vec<ResourceType> {
    //     match self {
    //         TheaterType::Temperate => [
    //             ResourceType::TheaterTemp,
    //             ResourceType::TilesTemp,
    //             ResourceType::BuildTemp,
    //         ]
    //         .to_vec(),
    //         TheaterType::Snow => [
    //             ResourceType::TheaterSnow,
    //             ResourceType::TilesSnow,
    //             ResourceType::BuildSnow,
    //         ]
    //         .to_vec(),
    //         TheaterType::Urban => [
    //             ResourceType::TheaterUrb,
    //             ResourceType::TilesUrb,
    //             ResourceType::BuildUrb,
    //         ]
    //         .to_vec(),
    //     }
    // }

    pub fn building_resource(&self) -> ResourceType {
        match self {
            TheaterType::Temperate => ResourceType::BuildTemp,
            TheaterType::Snow => ResourceType::BuildSnow,
            TheaterType::Urban => ResourceType::BuildUrb
        }
    }

    pub fn tiles_resource(&self) -> ResourceType {
        match self {
            TheaterType::Temperate => ResourceType::TilesTemp,
            TheaterType::Snow => ResourceType::TilesSnow,
            TheaterType::Urban => ResourceType::TilesUrb
        }
    }

    pub fn theater_resource(&self) -> ResourceType {
        match self {
            TheaterType::Temperate => ResourceType::TheaterTemp,
            TheaterType::Snow => ResourceType::TheaterSnow,
            TheaterType::Urban => ResourceType::TheaterUrb
        }
    }

    pub fn get_theater_type_by_str(theater_type: &str) -> TheaterType {
        match theater_type.to_lowercase().as_str() {
            "snow" => TheaterType::Snow,
            "temperate" => TheaterType::Temperate,
            "urban" => TheaterType::Urban,
            _ => panic!("未知的场景类型：{}", theater_type)
        }
    }

    // pub fn get_theater_setting_by_str(theater_type: &str) -> TheaterSetting {
    //     match theater_type {
    //         "Snow" => TheaterType::Snow.get_theater_setting(),
    //         "Temperate" => TheaterType::Temperate.get_theater_setting(),
    //         "Urban" => TheaterType::Urban.get_theater_setting(),
    //         _ => panic!("未知的场景类型：{}", theater_type),
    //     }
    // }
    // pub fn get_theater_name(&self) -> String {
    //     match self {
    //         Temperate => "Temperate".to_string(),
    //         Snow => "Snow".to_string(),
    //         Urban => "Urban".to_string(),
    //     }
    // }
    pub fn get_theater_setting(&self) -> TheaterSetting {
        match self {
            TheaterType::Temperate => TheaterSetting {
                theatertype:          TheaterType::Temperate,
                theater_ini:          "temperat.ini".to_string(),
                extension:            ".tile".to_string(),
                new_theater_char:     't',
                iso_palette_name:     "isotem.pal".to_string(),
                unit_palette_name:    "unittem.pal".to_string(),
                overlay_palette_name: "temperat.pal".to_string(),
                lib_palette_name:     "libtem.pal".to_string(),
                tiles_name:           "theater.temp".to_string(),
                shps_name:            "build.tem".to_string()
            },
            TheaterType::Snow => TheaterSetting {
                theatertype:          TheaterType::Snow,
                theater_ini:          "snow.ini".to_string(),
                extension:            ".tile".to_string(),
                new_theater_char:     'a',
                iso_palette_name:     "isosno.pal".to_string(),
                unit_palette_name:    "unitsno.pal".to_string(),
                overlay_palette_name: "snow.pal".to_string(),
                lib_palette_name:     "libsno.pal".to_string(),
                tiles_name:           "theater.snow".to_string(),
                shps_name:            "build.snow".to_string()
            },
            TheaterType::Urban => TheaterSetting {
                theatertype:          TheaterType::Urban,
                theater_ini:          "urban.ini".to_string(),
                extension:            ".tile".to_string(),
                new_theater_char:     'u',
                iso_palette_name:     "isourb.pal".to_string(),
                unit_palette_name:    "uniturb.pal".to_string(),
                overlay_palette_name: "urban.pal".to_string(),
                lib_palette_name:     "liburb.pal".to_string(),
                tiles_name:           "theater.urb".to_string(),
                shps_name:            "build.urb".to_string()
            }
        }
    }
}
#[derive(Clone)]
pub struct TheaterSetting {
    pub theatertype:          TheaterType,
    pub theater_ini:          String,
    pub extension:            String,
    pub new_theater_char:     char,
    pub iso_palette_name:     String,
    pub unit_palette_name:    String,
    pub overlay_palette_name: String,
    pub lib_palette_name:     String,
    pub tiles_name:           String,
    pub shps_name:            String
}
