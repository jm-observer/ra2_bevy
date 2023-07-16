use crate::{ini::IniSection, pub_use::*};
use std::sync::Arc;

#[derive(Clone, Debug, Resource)]
pub struct GeneralRules {
    pub base_unit: Vec<String>,
    pub build_speed: f32,
    pub buildup_time: f32,
    pub chrono_harv_too_far_distance: i32,
    ///悬崖-背部-不可通行
    pub cliff_back_impassability: i32,
    pub close_enough: f32,
    pub crew: CrewRules,
    pub engineer: String,
    pub game_speed_bias: i32,
    pub guard_area_targeting_delay: i32,
    pub harvester_too_far_distance: i32,
    pub lighting_warhead: String,
    pub low_power_penalty_modifier: i32,
    pub min_low_power_production_speed: f32,
    pub max_low_power_production_speed: f32,
    pub maximum_cheer_rate: i32,
    pub maximum_queued_objects: i32,
    pub max_waypoint_path_length: i32,
    pub multiple_factory: f32,
    pub normal_targeting_delay: i32,
    pub pad_aircraft: Vec<String>,
    pub radar: RadarRules,
    pub refund_percent: f32,
    pub repair: RepairRules,
    pub ship_sinking_weight: f32,
    pub technician: String,
    pub tree_strength: i32,
    pub veteran: VeteranRules,
    pub wall_build_speed_coefficient: f32,
    pub weather_con_bolt_explosion: String
}

impl GeneralRules {
    pub fn read_ini(val: Arc<IniSection>) -> Self {
        // println!("{:?}", val);
        let base_unit = val.get_array("BaseUnit");
        let build_speed = val.get_number_f32_from_str("BuildSpeed");
        let buildup_time = val.get_number_f32_from_str("BuildupTime");
        let chrono_harv_too_far_distance = val.get_number_i32_from_str("ChronoHarvTooFarDistance");
        let cliff_back_impassability = val.get_number_i32_from_str("CliffBackImpassability");
        let close_enough = val.get_number_f32_from_str("CloseEnough");
        let crew = CrewRules::read_ini(val.clone());
        let engineer = val.get_string("Engineer");
        let game_speed_bias = 1;
        let guard_area_targeting_delay = val.get_number_i32_from_str("GuardAreaTargetingDelay");
        let harvester_too_far_distance = val.get_number_i32_from_str("HarvesterTooFarDistance");
        let lighting_warhead = val.get_string("LightningWarhead");
        let low_power_penalty_modifier = val.get_number_i32_from_str("LowPowerPenaltyModifier");
        let min_low_power_production_speed =
            val.get_number_f32_from_str("MinLowPowerProductionSpeed");
        let max_low_power_production_speed =
            val.get_number_f32_from_str("MaxLowPowerProductionSpeed");
        let maximum_cheer_rate = val.get_number_i32_from_str("MaximumCheerRate");
        let maximum_queued_objects = val.get_number_i32_from_str("MaximumQueuedObjects");
        let max_waypoint_path_length = val.get_number_i32_from_str("MaxWaypointPathLength");
        let multiple_factory = val.get_number_f32_from_str("MultipleFactory");
        let normal_targeting_delay = val.get_number_i32_from_str("NormalTargetingDelay");
        let pad_aircraft = val.get_array("PadAircraft");
        let radar = RadarRules::read_ini(val.clone());
        let refund_percent = val.get_number_f32_from_str("RefundPercent");
        let repair = RepairRules::read_ini(val.clone());
        let ship_sinking_weight = val.get_number_f32_from_str("ShipSinkingWeight");
        let technician = val.get_string("Technician");
        let tree_strength = val.get_number_i32_from_str("TreeStrength");
        let veteran = VeteranRules::read_ini(val.clone());
        let wall_build_speed_coefficient = val.get_number_f32_from_str("WallBuildSpeedCoefficient");
        let weather_con_bolt_explosion = val.get_string("WeatherConBoltExplosion");

        GeneralRules {
            base_unit,
            build_speed,
            buildup_time,
            chrono_harv_too_far_distance,
            ///悬崖-背部-不可通行
            cliff_back_impassability,
            close_enough,
            crew,
            engineer,
            game_speed_bias,
            guard_area_targeting_delay,
            harvester_too_far_distance,
            lighting_warhead,
            low_power_penalty_modifier,
            min_low_power_production_speed,
            max_low_power_production_speed,
            maximum_cheer_rate,
            maximum_queued_objects,
            max_waypoint_path_length,
            multiple_factory,
            normal_targeting_delay,
            pad_aircraft,
            radar,
            refund_percent,
            repair,
            ship_sinking_weight,
            technician,
            tree_strength,
            veteran,
            wall_build_speed_coefficient,
            weather_con_bolt_explosion
        }
    }

    pub fn read_prereq_categories() {
        panic!("");
    }
}

///队，组?
#[derive(Clone, Debug)]
pub struct CrewRules {
    allied_crew:             String,
    allied_survivor_divisor: i32,
    crew_escape:             f32,
    soviet_crew:             String,
    soviet_survivor_divisor: i32,
    survivor_rate:           f32
}

impl CrewRules {
    pub fn read_ini(val: Arc<IniSection>) -> Self {
        CrewRules {
            allied_crew:             val.get_string("AlliedCrew"),
            allied_survivor_divisor: val.get_number_i32_from_str("AlliedSurvivorDivisor"),
            crew_escape:             val.get_number_f32_from_str("CrewEscape"),
            soviet_crew:             val.get_string("SovietCrew"),
            soviet_survivor_divisor: val.get_number_i32_from_str("SovietSurvivorDivisor"),
            survivor_rate:           val.get_number_f32_from_str("SurvivorRate")
        }
    }
}

#[derive(Clone, Debug)]
pub struct RadarRules {
    event_suppression_distances: Vec<String>,
    event_visibility_durations:  Vec<String>,
    event_durations:             Vec<f32>,
    flash_frame_time:            i32,
    combat_flash_time:           i32,
    event_min_radius:            i32,
    event_speed:                 f32,
    event_rotation_speed:        f32,
    event_color_speed:           f32
}

impl RadarRules {
    pub fn read_ini(val: Arc<IniSection>) -> Self {
        RadarRules {
            event_suppression_distances: val.get_array("RadarEventSuppressionDistances"),
            event_visibility_durations:  val.get_array("RadarEventVisibilityDurations"),
            event_durations:             val.get_number_array_defalut("RadarEventDurations"),
            flash_frame_time:            val.get_number_i32_from_str("FlashFrameTime"),
            combat_flash_time:           val.get_number_i32_from_str("RadarCombatFlashTime"),
            event_min_radius:            val.get_number_i32_from_str("RadarEventMinRadius"),
            event_speed:                 val.get_number_f32_from_str("RadarEventSpeed"),
            event_rotation_speed:        val.get_number_f32_from_str("RadarEventRotationSpeed"),
            event_color_speed:           val.get_number_f32_from_str("RadarEventColorSpeed")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RepairRules {
    repair_percent: f32,
    repair_rate:    f32,
    repair_step:    i32,
    u_repair_rate:  f32,
    i_repair_rate:  f32,
    i_repair_step:  i32
}
impl RepairRules {
    pub fn read_ini(val: Arc<IniSection>) -> Self {
        RepairRules {
            repair_percent: val.get_number_f32_from_str("RepairPercent"),
            repair_rate:    val.get_number_f32_from_str("RepairRate"),
            repair_step:    val.get_number_i32_from_str("RepairStep"),
            u_repair_rate:  val.get_number_f32_from_str("URepairRate"),
            i_repair_rate:  val.get_number_f32_from_str("IRepairRate"),
            i_repair_step:  val.get_number_i32_from_str("IRepairStep")
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct VeteranRules {
    veteran_ratio:   f32,
    veteran_combat:  f32,
    veteran_speed:   f32,
    veteran_sight:   f32,
    veteran_armor:   f32,
    veteran_rof:     f32,
    veteran_cap:     f32,
    initial_veteran: bool
}
impl VeteranRules {
    pub fn read_ini(val: Arc<IniSection>) -> Self {
        VeteranRules {
            veteran_ratio:   val.get_number_f32_from_str("VeteranRatio"),
            veteran_combat:  val.get_number_f32_from_str("VeteranCombat"),
            veteran_speed:   val.get_number_f32_from_str("VeteranSpeed"),
            veteran_sight:   1f32.max(val.get_number_f32_from_str("VeteranSight")),
            veteran_armor:   val.get_number_f32_from_str("VeteranArmor"),
            veteran_rof:     val.get_number_f32_from_str("VeteranROF"),
            veteran_cap:     val.get_number_f32_from_str("VeteranCap"),
            initial_veteran: val.get_bool("InitialVeteran")
        }
    }
}
