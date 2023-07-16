use crate::{ini::IniSection, ty::InfDeathType};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct WarheadRule {
    rules:                   Arc<IniSection>,
    verses:                  HashMap<i32, f32>,
    anim_list:               Vec<String>,
    bomb_disarm:             Option<bool>,
    bullets:                 Option<bool>,
    cell_spread:             Option<f32>,
    conventional:            Option<bool>,
    em_effect:               Option<bool>,
    inf_death:               i32,
    parasite:                Option<bool>,
    percent_at_max:          f32,
    prone_damage:            f32,
    temporal:                Option<bool>,
    wall_absolute_destroyer: Option<bool>,
    wall:                    Option<bool>,
    wood:                    Option<bool>
}

impl WarheadRule {
    pub fn new(ini: Arc<IniSection>) -> Self {
        let mut verses = HashMap::<i32, f32>::with_capacity(20);
        let anim_list = ini.get_array("AnimList");
        let bomb_disarm = ini.get_bool_option("BombDisarm");
        let bullets = ini.get_bool_option("Bullets");
        let cell_spread = ini.get_number_option("CellSpread");
        let conventional = ini.get_bool_option("Conventional");
        let em_effect = ini.get_bool_option("EMEffect");
        let inf_death = ini.get_number_i32_default("InfDeath", InfDeathType::None as i32);
        let parasite = ini.get_bool_option("Parasite");
        let percent_at_max = ini.get_number_default("PercentAtMax", 1.0);
        let prone_damage = ini.get_number_default("ProneDamage", 1.0);
        let temporal = ini.get_bool_option("Temporal");
        let wall_absolute_destroyer = ini.get_bool_option("WallAbsoluteDestroyer");
        let wall = ini.get_bool_option("Wall");
        let wood = ini.get_bool_option("Wood");

        let tmp = ini.get_number_array_defalut("Verses");
        for (index, val) in tmp.iter().enumerate() {
            verses.insert(index as i32, val.clone());
        }
        Self {
            rules: ini,
            verses,
            anim_list,
            bomb_disarm,
            bullets,
            cell_spread,
            conventional,
            em_effect,
            inf_death,
            parasite,
            percent_at_max,
            prone_damage,
            temporal,
            wall_absolute_destroyer,
            wall,
            wood
        }
    }

    pub fn get_name(&self) -> &str {
        self.rules.name.as_str()
    }
}
