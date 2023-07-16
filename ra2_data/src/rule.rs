mod aircraft;
mod building;
mod country;
mod general;
mod infantry;
mod land;
mod overlay;
mod projectile;
mod terrain;
mod tiberium;
mod vehicle;
mod warhead;

use crate::{
    ini::IniSection,
    map_object::ObjectType,
    ty::{
        ArmorType, BuildCat, FactoryType, LocomotorType, MovementZone, PipColor, SpeedType,
        VeteranAbility, WeaponType
    }
};
pub use aircraft::*;
pub use building::*;
pub use country::*;
pub use general::*;
pub use infantry::*;
pub use land::*;
pub use overlay::*;
pub use projectile::*;
pub use terrain::*;
pub use tiberium::*;
pub use vehicle::*;
pub use warhead::*;

use ra2_util::is_true;
use std::{marker::PhantomData, sync::Arc};

#[derive(Debug)]
pub struct TechnoRule<B> {
    mark:                     PhantomData<B>,
    // pub object_rule: ObjectRule,
    pub owner:                Vec<String>,
    // pub types: ObjectType,
    pub ini:                  Arc<IniSection>,
    pub alternate_arctic_art: Option<bool>,
    pub crushable:            bool,
    pub crush_sound:          String,
    pub dont_score:           Option<bool>,
    pub insignificant:        Option<bool>,
    pub legal_target:         bool,
    pub no_shadow:            Option<bool>,
    pub ui_name:              String,

    pub ai_base_planning_side: Option<i32>,
    pub required_houses: Vec<String>,
    pub forbidden_houses: Vec<String>,
    pub requires_stolen_allied_tech: Option<bool>,
    pub requires_stolen_soviet_tech: Option<bool>,
    pub tech_level: Option<f32>,
    pub cost: Option<i32>,
    pub power: Option<f32>,
    pub powered: Option<bool>,
    pub prerequisite: Vec<String>,
    pub soylent: Option<i32>,
    pub build_cat: i32,
    pub adjacent: Option<i32>,
    pub base_normal: bool,
    pub build_limit: Option<i32>,
    pub guard_range: Option<i32>,
    pub default_to_guard_area: Option<bool>,
    pub eligibile_for_ally_building: Option<bool>,
    pub number_impassable_rows: Option<i32>,
    pub bridge_repair_hut: Option<bool>,
    pub construction_yard: Option<bool>,
    pub refinery: Option<bool>,
    pub unit_repair: Option<bool>,
    pub is_base_defense: Option<bool>,
    pub super_weapon: String,
    pub naval: Option<bool>,
    pub underwater: Option<bool>,
    pub water_bound: Option<bool>,
    pub repairable: bool,
    pub click_repairable: bool,
    pub unsellable: Option<bool>,
    pub gdi_barracks: Option<bool>,
    pub nod_barracks: Option<bool>,
    pub number_of_docks: Option<i32>,
    pub weapons_factory: Option<bool>,
    pub land_targeting: Option<i32>,
    pub naval_targeting: Option<i32>,
    pub too_big_to_fit_under_bridge: bool,
    pub can_be_occupied: Option<bool>,
    pub max_number_occupants: Option<i32>,
    pub leave_rubble: Option<bool>,
    pub undeploys_into: String,
    pub deploys_into: String,
    pub capturable: Option<bool>,
    pub needs_engineer: Option<bool>,
    pub c4: Option<bool>,
    pub can_c4: bool,
    pub produce_cash_startup: Option<i32>,
    pub produce_cash_amount: Option<i32>,
    pub produce_cash_delay: Option<i32>,
    pub explosion: Vec<String>,
    pub explodes: Option<bool>,
    pub turret: Option<bool>,
    pub turret_count: i32,
    pub turret_anim: String,
    pub turret_anim_is_voxel: Option<bool>,
    pub turret_anim_x: Option<f32>,
    pub turret_anim_y: Option<f32>,
    pub turret_anim_z_adjust: Option<f32>,
    pub is_charge_turret: Option<bool>,
    pub free_unit: Option<String>,
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub elite_primary: Option<String>,
    pub elite_secondary: Option<String>,
    pub weapon_count: Option<i32>,
    pub death_weapon: String,
    pub death_weapon_damage_modifier: f32,
    pub veteran_abilities: Vec<i32>,
    pub elite_abilities: Vec<i32>,
    pub self_healing: Option<bool>,
    pub wall: Option<bool>,
    pub gate: Option<bool>,
    pub armor: i32,
    /// 血条
    pub strength: Option<i32>,
    pub immune: Option<bool>,
    pub type_immune: Option<bool>,
    pub is_tilter: bool,
    pub walk_rate: i32,
    pub idle_rate: i32,
    pub no_spawn_alt: Option<bool>,
    pub crusher: Option<bool>,
    pub considered_aircraft: Option<bool>,
    pub landable: Option<bool>,
    pub locomotor: LocomotorType,
    pub speed_type: Option<i32>,
    pub speed: Option<i32>,
    pub movement_zone: i32,
    pub fearless: Option<bool>,
    pub deployer: Option<bool>,
    pub deploy_fire: Option<bool>,
    pub deploy_fire_weapon: i32,
    pub fraidycat: bool,
    pub is_human: bool,
    pub organic: Option<bool>,
    pub occupier: Option<bool>,
    pub engineer: Option<bool>,
    pub civilian: Option<bool>,
    pub threat_posed: Option<i32>,
    pub special_threat_value: Option<i32>,
    pub can_passive_aquire: bool,
    pub can_retaliate: bool,
    pub prevent_attack_move: Option<bool>,
    pub opportunity_fire: Option<bool>,
    pub trainable: bool,
    pub crewed: Option<bool>,
    pub parasiteable: bool,
    pub reselect_if_limboed: Option<bool>,
    pub rejoin_team_if_limboed: Option<bool>,
    pub weight: Option<f32>,
    pub accelerates: Option<bool>,
    pub teleporter: Option<bool>,
    pub pip: i32,
    pub passengers: Option<i32>,
    pub ammo: Option<i32>,
    pub storage: Option<i32>,
    pub spawns: String,
    pub spawns_number: Option<i32>,
    pub missile_spawn: Option<bool>,
    pub size: i32,
    pub sight: i32,
    pub spy_sat: Option<bool>,
    pub harvester: Option<bool>,
    pub unloading_class: String,
    pub dock: Vec<String>,
    pub radar: Option<bool>,
    pub radar_invisible: Option<bool>,
    pub reveal_to_all: Option<bool>,
    pub selectable: bool,
    pub invisible_in_game: Option<bool>,
    pub move_to_shroud: bool,
    pub leadership_rating: i32,
    pub allowed_to_start_in_multiplayer: bool,
    pub rot: i32,
    pub factory: i32,
    pub is_lightpost: bool,
    pub light_visibility: i32,
    ///光线强度？
    pub light_intensity: Option<f32>,
    pub light_red_tint: f32,
    pub light_green_tint: f32,
    pub light_blue_tint: f32,
    pub ambient_sound: Option<String>,
    pub create_sound: Option<String>,
    pub deploy_sound: Option<String>,
    pub undeploy_sound: Option<String>,
    pub voice_select: Option<String>,
    pub voice_move: Option<String>,
    pub voice_attack: Option<String>,
    pub voice_feedback: Option<String>,
    pub voice_special_attack: Option<String>,
    pub voice_enter: Option<String>,
    pub voice_capture: Option<String>,
    pub die_sound: Option<String>,
    pub move_sound: Option<String>,
    pub enter_water_sound: Option<String>,
    pub leave_water_sound: Option<String>,
    pub turret_rotate_sound: Option<String>,
    pub working_sound: Option<String>,
    pub not_working_sound: Option<String>,
    pub chrono_in_sound: Option<String>,
    pub chrono_out_sound: Option<String>
}
impl<B> TechnoRule<B> {
    pub fn new(types: ObjectType, ini: Arc<IniSection>) -> Self {
        let alternate_arctic_art = ini.get_bool_option("AlternateArcticArt");
        let crushable = ini.get_bool_or_default("Crushable", types == ObjectType::Infantry);
        let crush_sound = ini.get_string("CrushSound");
        let dont_score = ini.get_bool_option("DontScore");
        let insignificant = ini.get_bool_option("Insignificant");
        let legal_target = ini.get_bool_or_default("LegalTarget", true);
        let no_shadow = ini.get_bool_option("NoShadow");
        let ui_name = ini.get_string("UIName");
        let owner = ini.get_array("Owner");
        let ai_base_planning_side = ini.get_number_i32_from_str_option("AIBasePlanningSide");
        let required_houses = ini.get_array("RequiredHouses");
        let forbidden_houses = ini.get_array("ForbiddenHouses");
        let requires_stolen_allied_tech = ini.get_bool_option("RequiresStolenAlliedTech");
        let requires_stolen_soviet_tech = ini.get_bool_option("RequiresStolenSovietTech");
        let tech_level = ini.get_number_f32_from_str_option("TechLevel");
        let cost = ini.get_number_i32_from_str_option("Cost");
        let power = ini.get_number_f32_from_str_option("Power");
        let powered = ini.get_bool_option("Powered");
        let prerequisite = ini.get_array("Prerequisite");
        let soylent = ini.get_number_i32_from_str_option("Soylent");
        let build_cat = ini.get_enum("BuildCat", BuildCat::Combat, false);
        let adjacent = ini.get_number_i32_from_str_option("Adjacent");
        let base_normal = ini.get_bool_or_default("BaseNormal", true);
        let build_limit = ini.get_number_i32_from_str_option("BuildLimit");
        let guard_range = ini.get_number_i32_from_str_option("GuardRange");
        let default_to_guard_area = ini.get_bool_option("DefaultToGuardArea");
        let eligibile_for_ally_building = ini.get_bool_option("EligibileForAllyBuilding");
        let number_impassable_rows = ini.get_number_i32_from_str_option("NumberImpassableRows");
        let bridge_repair_hut = ini.get_bool_option("BridgeRepairHut");
        let construction_yard = ini.get_bool_option("ConstructionYard");
        let refinery = ini.get_bool_option("Refinery");
        let unit_repair = ini.get_bool_option("UnitRepair");
        let is_base_defense = ini.get_bool_option("IsBaseDefense");
        let super_weapon = ini.get_string("SuperWeapon");
        let t = ini.get_bool_option("Naval");
        let naval = t;
        let underwater = ini.get_bool_option("Underwater");
        let water_bound = ini.get_bool_option("WaterBound");
        let repairable = ini.get_bool_or_default("Repairable", types == ObjectType::Building);
        let click_repairable =
            ini.get_bool_or_default("ClickRepairable", types == ObjectType::Building);
        let unsellable = ini.get_bool_option("Unsellable");
        let gdi_barracks = ini.get_bool_option("GDIBarracks");
        let nod_barracks = ini.get_bool_option("NODBarracks");
        let mut number_of_docks = ini.get_number_i32_option("NumberOfDocks");
        if is_true(unit_repair) && number_of_docks.is_none() {
            number_of_docks = Some(1);
        }
        let mut factory = ini.get_enum("Factory", FactoryType::None, false);
        if factory == FactoryType::UnitType as i32 && is_true(t) {
            factory = FactoryType::NavalUnitType as i32;
        }
        let weapons_factory = ini.get_bool_option("WeaponsFactory");
        let land_targeting = ini.get_number_i32_option("LandTargeting");
        let naval_targeting = ini.get_number_i32_option("NavalTargeting");
        let too_big_to_fit_under_bridge =
            ini.get_bool_or_default("TooBigToFitUnderBridge", types == ObjectType::Building);
        let can_be_occupied = ini.get_bool_option("CanBeOccupied");
        let max_number_occupants = ini.get_number_i32_option("MaxNumberOccupants");
        let leave_rubble = ini.get_bool_option("LeaveRubble");
        let undeploys_into = ini.get_string("UndeploysInto");
        let deploys_into = ini.get_string("DeploysInto");
        let capturable = ini.get_bool_option("Capturable");
        let needs_engineer = ini.get_bool_option("NeedsEngineer");
        let c4 = ini.get_bool_option("C4");
        let can_c4 = ini.get_bool_or_default("CanC4", true);
        let produce_cash_startup = ini.get_number_i32_option("ProduceCashStartup");
        let produce_cash_amount = ini.get_number_i32_option("ProduceCashAmount");
        let produce_cash_delay = ini.get_number_i32_option("ProduceCashDelay");
        let explosion = ini.get_array("Explosion");
        let explodes = ini.get_bool_option("Explodes");
        let turret = ini.get_bool_option("Turret");
        let default = if is_true(turret) { 1i32 } else { 0i32 };
        let turret_count = ini.get_number_i32_default("TurretCount", default);
        let turret_anim = ini.get_string("TurretAnim");
        let turret_anim_is_voxel = ini.get_bool_option("TurretAnimIsVoxel");
        let turret_anim_x = ini.get_number_option("TurretAnimX");
        let turret_anim_y = ini.get_number_option("TurretAnimY");
        let turret_anim_z_adjust = ini.get_number_option("TurretAnimZAdjust");
        let is_charge_turret = ini.get_bool_option("IsChargeTurret");
        let free_unit = ini.get_string_option("FreeUnit");
        let primary = ini.get_string_option("Primary");
        let secondary = ini.get_string_option("Secondary");
        let elite_primary = ini.get_string_option("ElitePrimary");
        let elite_secondary = ini.get_string_option("EliteSecondary");
        let weapon_count = ini.get_number_i32_option("WeaponCount");
        let death_weapon = ini.get_string("DeathWeapon");
        let death_weapon_damage_modifier = ini.get_number_default("DeathWeaponDamageModifier", 1.0);
        let veteran_abilities =
            ini.get_enum_array_default("VeteranAbilities", VeteranAbility::FASTER);
        let mut elite_abilities =
            ini.get_enum_array_default("EliteAbilities", VeteranAbility::FASTER);
        elite_abilities.append(&mut veteran_abilities.clone());
        let self_healing = ini.get_bool_option("SelfHealing");
        let wall = ini.get_bool_option("Wall");
        let gate = ini.get_bool_option("Gate");
        let armor = ini.get_enum("Armor", ArmorType::None, true);
        let strength = ini.get_number_i32_option("Strength");
        let immune = ini.get_bool_option("Immune");
        let type_immune = ini.get_bool_option("TypeImmune");
        let is_tilter = ini.get_bool_or_default("IsTilter", true);
        let walk_rate = ini.get_number_i32_default("WalkRate", 1);
        let idle_rate = ini.get_number_i32_default("IdleRate", 1);
        let no_spawn_alt = ini.get_bool_option("NoSpawnAlt");
        let crusher = ini.get_bool_option("Crusher");
        let considered_aircraft = ini.get_bool_option("ConsideredAircraft");
        let landable = ini.get_bool_option("Landable");
        let y = ini.get_string_option("Locomotor");
        let locomotor: LocomotorType;
        if let Some(y) = y {
            let e = LocomotorType::locomotor_types_by_cls_id(y.as_str());
            if let Some(e) = e {
                locomotor = e;
            } else {
                //todo warn
                locomotor = LocomotorType::Statue;
            }
        } else {
            locomotor = LocomotorType::Statue;
        }
        let mut speed_type: Option<i32> = None;
        if locomotor != LocomotorType::Statue {
            let mut e = LocomotorType::default_speeds_by_locomotor(locomotor);
            if e.is_none() {
                e = Some(
                    if types == ObjectType::Aircraft || is_true(considered_aircraft) {
                        SpeedType::Winged
                    } else if types == ObjectType::Vehicle {
                        if is_true(crusher) {
                            SpeedType::Track
                        } else {
                            SpeedType::Wheel
                        }
                    } else if types == ObjectType::Infantry {
                        SpeedType::Foot
                    } else {
                        panic!()
                    }
                );
            }
            let mut speed_type_tmp = ini.get_enum("SpeedType", e.unwrap(), false);
            if is_true(considered_aircraft) && (speed_type_tmp == (SpeedType::Hover as i32)) {
                speed_type_tmp = SpeedType::Winged as i32;
            }
            speed_type = Some(speed_type_tmp);
        }
        let mut speed = None;
        if let Some(speed_tmp) = ini.get_number_i32_option("Speed") {
            speed = Some(255 * speed_tmp / 64);
        }
        let movement_zone = ini.get_enum("MovementZone", MovementZone::Normal, false);
        let fearless = ini.get_bool_option("Fearless");
        let deployer = ini.get_bool_option("Deployer");
        let deploy_fire = ini.get_bool_option("DeployFire");
        let deploy_fire_weapon =
            ini.get_number_i32_default("DeployFireWeapon", WeaponType::Secondary as i32);
        let fraidycat = ini.get_bool_or_default("Fraidycat", false);
        let is_human = !is_true(ini.get_bool_option("NotHuman"));
        let organic = ini.get_bool_option("Organic");
        let occupier = ini.get_bool_option("Occupier");
        let engineer = ini.get_bool_option("Engineer");
        let civilian = ini.get_bool_option("Civilian");
        let threat_posed = ini.get_number_i32_option("ThreatPosed");
        let special_threat_value = ini.get_number_i32_option("SpecialThreatValue");
        let can_passive_aquire = ini.get_bool_or_default("CanPassiveAquire", true);
        let can_retaliate = ini.get_bool_or_default("CanRetaliate", true);
        let prevent_attack_move = ini.get_bool_option("PreventAttackMove");
        let opportunity_fire = ini.get_bool_option("OpportunityFire");
        let trainable = ini.get_bool_or_default("Trainable", types != ObjectType::Building);
        let crewed = ini.get_bool_option("Crewed");
        let parasiteable = ini.get_bool_or_default("Parasiteable", true);
        let reselect_if_limboed = ini.get_bool_option("ReselectIfLimboed");
        let rejoin_team_if_limboed = ini.get_bool_option("RejoinTeamIfLimboed");
        let weight = ini.get_number_option("Weight");
        let mut accelerates = ini.get_bool_option("Accelerates");
        if accelerates.is_none() {
            if let Some(weight) = weight {
                accelerates = Some(weight > 1f32)
            }
        }
        let teleporter = ini.get_bool_option("Teleporter");
        let pip = ini.get_enum("Pip", PipColor::Green, true);
        let passengers = ini.get_number_i32_option("Passengers");
        let ammo = ini.get_number_i32_option("Ammo");
        let storage = ini.get_number_i32_option("Storage");
        let spawns = ini.get_string("Spawns");
        let spawns_number = ini.get_number_i32_option("SpawnsNumber");
        let missile_spawn = ini.get_bool_option("MissileSpawn");
        let size = ini.get_number_i32_default("Size", 1);
        let sight = 11i32.min(ini.get_number_i32_from_str_default("Sight", 1));
        let spy_sat = ini.get_bool_option("SpySat");
        let harvester = ini.get_bool_option("Harvester");
        let unloading_class = ini.get_string("UnloadingClass");
        let dock = ini.get_array("Dock");
        let radar = ini.get_bool_option("Radar");
        let radar_invisible = ini.get_bool_option("RadarInvisible");
        let reveal_to_all = ini.get_bool_option("RevealToAll");
        let selectable = ini.get_bool_or_default("Selectable", true);
        let invisible_in_game = ini.get_bool_option("InvisibleInGame");
        let move_to_shroud = ini.get_bool_or_default("MoveToShroud", types != ObjectType::Aircraft);
        let leadership_rating = ini.get_number_i32_default("LeadershipRating", 5);
        let allowed_to_start_in_multiplayer =
            ini.get_bool_or_default("AllowedToStartInMultiplayer", true);
        let rot = ini.get_number_i32_from_str_default("ROT", 0) / 256 * 360;

        let e = ini.get_string_option("Image");
        let is_lightpost = if let Some(e) = e {
            "GALITE" == e.as_str()
        } else {
            "GALITE" == ini.name.as_str()
        };
        let light_visibility = ini.get_number_i32_from_str_default("LightVisibility", 5000);
        let light_intensity = ini.get_number_option("LightIntensity");
        let light_red_tint = ini.get_number_default("LightRedTint", 1.0);
        let light_green_tint = ini.get_number_default("LightGreenTint", 1.0);
        let light_blue_tint = ini.get_number_default("LightBlueTint", 1.0);
        let ambient_sound = ini.get_string_option("AmbientSound");
        let create_sound = ini.get_string_option("CreateSound");
        let deploy_sound = ini.get_string_option("DeploySound");
        let undeploy_sound = ini.get_string_option("UndeploySound");
        let voice_select = ini.get_string_option("VoiceSelect");
        let voice_move = ini.get_string_option("VoiceMove");
        let voice_attack = ini.get_string_option("VoiceAttack");
        let voice_feedback = ini.get_string_option("VoiceFeedback");
        let voice_special_attack = ini.get_string_option("VoiceSpecialAttack");
        let voice_enter = ini.get_string_option("VoiceEnter");
        let voice_capture = ini.get_string_option("VoiceCapture");
        let die_sound = ini.get_string_option("DieSound");
        let move_sound = ini.get_string_option("MoveSound");
        let enter_water_sound = ini.get_string_option("EnterWaterSound");
        let leave_water_sound = ini.get_string_option("LeaveWaterSound");
        let turret_rotate_sound = ini.get_string_option("TurretRotateSound");
        let working_sound = ini.get_string_option("WorkingSound");
        let not_working_sound = ini.get_string_option("NotWorkingSound");
        let chrono_in_sound = ini.get_string_option("ChronoInSound");
        let chrono_out_sound = ini.get_string_option("ChronoOutSound");

        // panic!();PhantomData<B>
        Self {
            mark: PhantomData,
            owner,
            // types,
            ini,
            alternate_arctic_art,
            crushable,
            crush_sound,
            dont_score,
            insignificant,
            legal_target,
            no_shadow,
            ui_name,
            ai_base_planning_side,
            required_houses,
            forbidden_houses,
            requires_stolen_allied_tech,
            requires_stolen_soviet_tech,
            tech_level,
            cost,
            power,
            powered,
            prerequisite,
            soylent,
            build_cat,
            adjacent,
            factory,
            base_normal,
            build_limit,
            guard_range,
            default_to_guard_area,
            eligibile_for_ally_building,
            number_impassable_rows,
            bridge_repair_hut,
            construction_yard,
            refinery,
            unit_repair,
            is_base_defense,
            super_weapon,
            naval,
            underwater,
            water_bound,
            repairable,
            click_repairable,
            unsellable,
            gdi_barracks,
            nod_barracks,
            number_of_docks,
            weapons_factory,
            land_targeting,
            naval_targeting,
            too_big_to_fit_under_bridge,
            can_be_occupied,
            max_number_occupants,
            leave_rubble,
            undeploys_into,
            deploys_into,
            capturable,
            needs_engineer,
            c4,
            can_c4,
            produce_cash_startup,
            produce_cash_amount,
            produce_cash_delay,
            explosion,
            explodes,
            turret,
            turret_count,
            turret_anim,
            turret_anim_is_voxel,
            turret_anim_x,
            turret_anim_y,
            turret_anim_z_adjust,
            is_charge_turret,
            free_unit,
            primary,
            secondary,
            elite_primary,
            elite_secondary,
            weapon_count,
            death_weapon,
            death_weapon_damage_modifier,
            veteran_abilities,
            elite_abilities,
            self_healing,
            wall,
            gate,
            armor,
            strength,
            immune,
            type_immune,
            is_tilter,
            walk_rate,
            idle_rate,
            no_spawn_alt,
            crusher,
            considered_aircraft,
            landable,
            locomotor,
            speed_type,
            speed,
            movement_zone,
            fearless,
            deployer,
            deploy_fire,
            deploy_fire_weapon,
            fraidycat,
            is_human,
            organic,
            occupier,
            engineer,
            civilian,
            threat_posed,
            special_threat_value,
            can_passive_aquire,
            can_retaliate,
            prevent_attack_move,
            opportunity_fire,
            trainable,
            crewed,
            parasiteable,
            reselect_if_limboed,
            rejoin_team_if_limboed,
            weight,
            accelerates,
            teleporter,
            pip,
            passengers,
            ammo,
            storage,
            spawns,
            spawns_number,
            missile_spawn,
            size,
            sight,
            spy_sat,
            harvester,
            unloading_class,
            dock,
            radar,
            radar_invisible,
            reveal_to_all,
            selectable,
            invisible_in_game,
            move_to_shroud,
            leadership_rating,
            allowed_to_start_in_multiplayer,
            rot,

            is_lightpost,
            light_visibility,
            light_intensity,
            light_red_tint,
            light_green_tint,
            light_blue_tint,
            ambient_sound,
            create_sound,
            deploy_sound,
            undeploy_sound,
            voice_select,
            voice_move,
            voice_attack,
            voice_feedback,
            voice_special_attack,
            voice_enter,
            voice_capture,
            die_sound,
            move_sound,
            enter_water_sound,
            leave_water_sound,
            turret_rotate_sound,
            working_sound,
            not_working_sound,
            chrono_in_sound,
            chrono_out_sound
        }
    }

    pub fn get_name(&self) -> &str {
        self.ini.name.as_str()
    }

    pub fn get_image_name(&self) -> String {
        let e = self.ini.get_string_option("Image");
        if let Some(e) = e {
            e
        } else {
            self.get_name().to_string()
        }
    }
}
