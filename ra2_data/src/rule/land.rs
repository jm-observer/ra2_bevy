use crate::{ini::IniSection, ty::SpeedType};
use std::{collections::HashMap, sync::Arc};

/// 地形对于各种移动类型的影响（1则无影响，小于1则为一定影响）
#[derive(Debug)]
pub struct LandRule {
    pub build_able:      bool,
    pub speed_modifiers: HashMap<SpeedType, f32>
}
impl LandRule {
    pub fn new(ini: Arc<IniSection>) -> Self {
        let build_able = ini.get_bool_or_default("Buildable", false);
        let speed_modifiers: HashMap<SpeedType, f32> = ini
            .entries
            .iter()
            .map(|(name, entry)| {
                let speed_type = SpeedType::get_by_str(name.as_str());
                // let speed = entry.get_str().parse::<f32>().unwrap();
                (speed_type, entry)
            })
            .filter(|(name, _entry)| name.is_some())
            .map(|(name, entry)| {
                let name = name.unwrap();
                (name, entry.as_str().unwrap().parse::<f32>().unwrap())
            })
            .collect();
        Self {
            build_able,
            speed_modifiers
        }
    }

    pub fn get_speed_modifier(&self, e: &SpeedType) -> f32 {
        if e == &SpeedType::Foot && self.get_speed_modifier(&SpeedType::Track) == 0.0 {
            return 0.0;
        }
        if let Some(t) = self.speed_modifiers.get(e) {
            return *t;
        } else {
            return 1.0;
        }
    }
}
