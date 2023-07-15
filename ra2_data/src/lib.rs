pub mod color;
pub mod coord;
pub mod countries;
mod extend_json;
pub mod ini;
pub mod lighting;
pub mod map;
pub mod map_object;
pub mod overlay;
pub mod pub_use;
pub mod resource;
pub mod shp;
pub mod size;
pub mod structure;
pub mod theater;
pub mod tile;
pub mod vxl;

pub trait GetEnum {
    fn get_num_by_str(&self, key: &str) -> Option<i32>;
    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32>;
    fn get_num(&self) -> i32;
}

pub fn is_value_array(key: &str) -> bool {
    match key {
        "BuildingTypes" | "AircraftTypes" | "InfantryTypes" | "OverlayTypes" | "TerrainTypes"
        | "SmudgeTypes" | "VehicleTypes" => true,
        _ => false
    }
}
