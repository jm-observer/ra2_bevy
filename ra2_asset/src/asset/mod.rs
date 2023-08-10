mod ini;
mod map;
mod palette;
mod shp;
mod tile;
mod vxl;

pub use ini::*;
pub use map::*;
pub use palette::*;
pub use shp::*;
pub use tile::*;
pub use vxl::*;

pub fn is_value_array(key: &str) -> bool {
    match key {
        "BuildingTypes" | "AircraftTypes" | "InfantryTypes" | "OverlayTypes" | "TerrainTypes"
        | "SmudgeTypes" | "VehicleTypes" => true,
        _ => false
    }
}
