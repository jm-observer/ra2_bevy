mod ini;
mod palette;
mod vxl;

pub use ini::*;
pub use palette::*;
pub use vxl::*;

pub fn is_value_array(key: &str) -> bool {
    match key {
        "BuildingTypes" | "AircraftTypes" | "InfantryTypes" | "OverlayTypes" | "TerrainTypes"
        | "SmudgeTypes" | "VehicleTypes" => true,
        _ => false
    }
}
