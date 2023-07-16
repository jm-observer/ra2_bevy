use crate::{countries::Countries, ini::IniSection, pub_use::*};
use num_derive::FromPrimitive;
#[derive(Debug, Resource)]
pub struct CountryRule {
    name:              &'static str,
    ui_name:           String,
    side:              SideType,
    multiplay:         bool,
    multiplay_passive: bool
}

impl CountryRule {
    pub fn new(ini: Arc<IniSection>) -> Self {
        let name = Countries::get_contry_name(&ini.name);
        let ui_name = ini.get_string("UIName");
        let t = ini.get_string_option("Side");
        let side: SideType;
        if let Some(t) = t {
            side = Self::get_side_type(t.as_str());
            let multiplay = ini.get_bool_or_default("Multiplay", false);
            let multiplay_passive = ini.get_bool_or_default("MultiplayPassive", false);
            CountryRule {
                name,
                ui_name,
                side,
                multiplay,
                multiplay_passive
            }
        } else {
            panic!();
        }
    }

    fn get_side_type(key: &str) -> SideType {
        match key {
            "GDI" => SideType::GDI,
            "Nod" => SideType::Nod,
            "Civilian" => SideType::Civilian,
            "Mutant" => SideType::Mutant,
            _ => panic!()
        }
    }
}

///?
#[derive(FromPrimitive, Debug)]
pub enum SideType {
    ///盟军
    GDI      = 0,
    ///苏俄
    Nod      = 1,
    ///中立
    Civilian = 2,
    Mutant   = 3
}
