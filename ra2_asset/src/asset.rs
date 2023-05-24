mod ini;

pub use ini::*;

pub fn is_value_array(key: &str) -> bool {
    match key {
        "BuildingTypes" | "AircraftTypes" | "InfantryTypes"
        | "OverlayTypes" | "TerrainTypes" | "SmudgeTypes"
        | "VehicleTypes" => true,
        _ => false
    }
}
