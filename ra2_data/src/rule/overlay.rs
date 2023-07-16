use crate::{ini::IniSection, ty::LandType};
use std::sync::Arc;

#[derive(Debug)]
pub struct OverlayRule {
    // pub object_rule: ObjectRule,
    pub name:                  String,
    pub armor:                 Vec<String>,
    pub is_arock:              bool,
    pub is_rubble:             bool,
    pub is_veinhole_monster:   bool,
    pub is_veins:              bool,
    pub land:                  i32,
    pub no_use_tile_land_type: bool,
    pub strength:              Option<i32>,
    pub tiberium:              bool,
    pub wall:                  bool,
    pub radar_invisible:       bool,

    pub ini:                  Arc<IniSection>,
    pub alternate_arctic_art: Option<bool>,
    pub crushable:            bool,
    pub crush_sound:          String,
    pub dont_score:           Option<bool>,
    pub insignificant:        Option<bool>,
    pub legal_target:         bool,
    pub no_shadow:            Option<bool>,
    pub ui_name:              String,
    image:                    Option<String>,
    pub chain_reaction:       bool
}

impl OverlayRule {
    pub fn new(name: String, ini: Arc<IniSection>) -> Self {
        let chain_reaction = ini.get_bool_or_default("ChainReaction", false);
        let image = ini.get_string_option("Image");
        let alternate_arctic_art = ini.get_bool_option("AlternateArcticArt");
        let crushable = ini.get_bool_or_default("Crushable", false);
        let crush_sound = ini.get_string("CrushSound");
        let dont_score = ini.get_bool_option("DontScore");
        let insignificant = ini.get_bool_option("Insignificant");
        let legal_target = ini.get_bool_or_default("LegalTarget", true);
        let no_shadow = ini.get_bool_option("NoShadow");
        let ui_name = ini.get_string("UIName");

        let armor = ini.get_array("Armor");
        let is_arock = ini.get_bool_or_default("IsARock", false);
        let is_rubble = ini.get_bool_or_default("IsRubble", false);

        let is_veinhole_monster = ini.get_bool_or_default("IsVeinholeMonster", false);
        let is_veins = ini.get_bool_or_default("IsVeins", false);
        let land = ini.get_enum("Land", LandType::Clear, false);

        let no_use_tile_land_type = ini.get_bool_or_default("NoUseTileLandType", false);
        let strength = ini.get_number_i32_option("Strength");
        let tiberium = ini.get_bool_or_default("Tiberium", false);
        let wall = ini.get_bool_or_default("Wall", false);
        let radar_invisible = ini.get_bool_or_default("RadarInvisible", false);

        Self {
            name,
            ini,
            alternate_arctic_art,
            crushable,
            crush_sound,
            dont_score,
            insignificant,
            legal_target,
            no_shadow,
            ui_name,

            armor,
            is_arock,
            is_rubble,
            is_veinhole_monster,
            is_veins,
            land,
            no_use_tile_land_type,
            strength,
            tiberium,
            wall,
            radar_invisible,
            image,
            chain_reaction
        }
    }

    pub fn get_image(&self) -> &str {
        if let Some(image) = &self.image {
            image.as_str()
        } else {
            self.name.as_str()
        }
    }
}
