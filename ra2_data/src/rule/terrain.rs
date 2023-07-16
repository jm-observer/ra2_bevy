use crate::{ini::IniSection, map_object::ObjectType};
use std::sync::Arc;

#[derive(Debug)]
pub struct TerrainRule {
    pub animation_rate:        Option<i32>,
    pub animation_probability: Option<f32>,
    pub gate:                  Option<bool>,
    pub immune:                Option<bool>,
    pub spawns_tiberium:       Option<bool>,
    pub strength:              Option<i32>,
    pub radar_invisible:       Option<bool>,

    pub ini:                  Arc<IniSection>,
    pub alternate_arctic_art: Option<bool>,
    pub crushable:            bool,
    pub crush_sound:          String,
    pub dont_score:           Option<bool>,
    pub insignificant:        Option<bool>,
    pub legal_target:         bool,
    pub no_shadow:            Option<bool>,
    pub ui_name:              String,
    pub name:                 String,
    image:                    Option<String>
}
impl TerrainRule {
    pub fn new(types: ObjectType, ini: Arc<IniSection>) -> Self {
        let name = ini.name.clone();
        let image: Option<String> = ini.get_string_option("image");
        let alternate_arctic_art = ini.get_bool_option("AlternateArcticArt");
        let crushable = ini.get_bool_or_default("Crushable", types == ObjectType::Infantry);
        let crush_sound = ini.get_string("CrushSound");
        let dont_score = ini.get_bool_option("DontScore");
        let insignificant = ini.get_bool_option("Insignificant");
        let legal_target = ini.get_bool_or_default("LegalTarget", true);
        let no_shadow = ini.get_bool_option("NoShadow");
        let ui_name = ini.get_string("UIName");

        let animation_rate = ini.get_number_i32_option("AnimationRate");
        let animation_probability = ini.get_number_option("AnimationProbability");
        let gate = ini.get_bool_option("Gate");
        let immune = ini.get_bool_option("Immune");

        let spawns_tiberium = ini.get_bool_option("SpawnsTiberium");
        let strength = ini.get_number_i32_option("Strength");
        let radar_invisible = ini.get_bool_option("RadarInvisible");

        Self {
            ini,
            alternate_arctic_art,
            crushable,
            crush_sound,
            dont_score,
            insignificant,
            legal_target,
            no_shadow,
            ui_name,

            animation_rate,
            animation_probability,
            gate,
            immune,
            spawns_tiberium,
            strength,
            radar_invisible,
            image,
            name
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
