use crate::asset::is_value_array;
use anyhow::{anyhow, bail, Result};
use bevy::{
    log::warn,
    reflect::TypeUuid,
    utils::HashMap
};
use configparser::ini::Ini;
use ra2_data::config::IniSection;
use ra2_util::get_value_map;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "b0298126-fc12-4767-8d0f-ddab3da2ebf8"]
pub struct IniFile {
    pub name:     String,
    pub sections: HashMap<String, Arc<IniSection>>
}

impl IniFile {
    pub fn new(
        name: String,
        val: Value,
        ini: Option<Ini>
    ) -> Result<Self> {
        // let name = string_to_static_str(name);
        let secs = get_value_map(val)?;
        let mut sections =
            HashMap::<String, IniSection>::with_capacity(
                secs.len() + 10
            );
        for (key, val) in secs {
            sections.insert(key.clone(), IniSection::new(val, key)?);
        }
        //mergeWith
        if let Some(ini) = ini {
            let maps = ini.get_map().ok_or(anyhow!("none"))?;
            for (key, map) in maps {
                if !sections.contains_key(key.as_str()) {
                    sections.insert(
                        key.clone(),
                        IniSection::new_empty(key.clone())
                    );
                };
                let tmp = sections.get_mut(key.as_str()).ok_or(
                    anyhow!("sections not contail: {}", key)
                )?;
                if is_value_array(key.as_str()) {
                    bail!("should not be array");
                } else {
                    for (key, val) in map {
                        let Some(val) = val else {
                            warn!("key {} is none", key);
                            continue
                        };
                        tmp.set(key, val);
                    }
                }
            }
        }
        let sections = sections
            .into_iter()
            .map(|(key, val)| (key, Arc::new(val)))
            .collect::<HashMap<String, Arc<IniSection>>>();
        Ok(IniFile { name, sections })
    }

    pub fn get_or_create_section(
        &mut self,
        key: &str
    ) -> Arc<IniSection> {
        let Some(section) = self.sections.get(key) else {
            let tmp =
                Arc::new(IniSection::new_empty(key.to_string()));
            self.sections.insert(key.to_string(), tmp.clone());
            return tmp;
        };
        section.clone()
    }

    pub fn get_section(&self, key: &str) -> Result<Arc<IniSection>> {
        //rules.ini去掉了NAWAST "GARADR" CAIRSFGL "NAHPAD"
        // debug!("name={}, key={}", &self.name, key);
        if let Some(section) = self.sections.get(key) {
            Ok(section.clone())
        } else {
            bail!(
                "get_section fail: name={}, key={}",
                &self.name,
                key
            );
        }
    }

    pub fn get_section_option(
        &self,
        key: &str
    ) -> Option<&Arc<IniSection>> {
        //rules.ini去掉了NAWAST "GARADR" CAIRSFGL "NAHPAD"
        // debug!("name={}, key={}", &self.name, key);
        self.sections.get(key)
    }

    pub fn get_ordered_sections(
        &self
    ) -> &HashMap<String, Arc<IniSection>> {
        &self.sections
    }
}
