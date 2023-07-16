use crate::ini::IniSection;
use std::sync::Arc;

#[derive(Debug)]
pub struct ProjectileRule {
    //ObjectRules
    pub arcing:          bool,
    pub firers_palette:  bool,
    pub inviso:          bool,
    pub is_anti_air:     bool,
    pub is_anit_ground:  bool,
    pub rot:             i32,
    pub shraphel_weapon: String,
    pub shrapnel_count:  i32,

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

    pub name: String,
    image:    Option<String>
}

impl ProjectileRule {
    pub fn new(ini: Arc<IniSection>) -> Self {
        let alternate_arctic_art = ini.get_bool_option("AlternateArcticArt");
        let crushable = ini.get_bool_or_default("Crushable", false);
        let crush_sound = ini.get_string("CrushSound");
        let dont_score = ini.get_bool_option("DontScore");
        let insignificant = ini.get_bool_option("Insignificant");
        let legal_target = ini.get_bool_or_default("LegalTarget", true);
        let no_shadow = ini.get_bool_option("NoShadow");
        let ui_name = ini.get_string("UIName");

        let arcing = ini.get_bool("Arcing");
        let firers_palette = ini.get_bool("FirersPalette");
        let inviso = ini.get_bool("Inviso");
        let is_anti_air = ini.get_bool("AA");
        let is_anit_ground = ini.get_bool_or_default("NoShadow", true);
        let rot = ini.get_number_i32_default("ROT", 0) / 256 * 360;
        let shraphel_weapon = ini.get_string("ShrapnelWeapon");
        let shrapnel_count = ini.get_number_i32("ShrapnelCount");

        let name = ini.name.clone();
        let image: Option<String> = ini.get_string_option("image");
        Self {
            name,
            image,
            ini,
            alternate_arctic_art,
            crushable,
            crush_sound,
            dont_score,
            insignificant,
            legal_target,
            no_shadow,
            ui_name,

            arcing,
            firers_palette,
            inviso,
            is_anti_air,
            is_anit_ground,
            rot,
            shraphel_weapon,
            shrapnel_count
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
