use crate::{is_value_array, pub_use::*, GetEnum};
use bevy::log::error;
use configparser::ini::Ini;
use ra2_util::{get_value_map, split_by_regex, DEFAULT_REGEX, GET_NUMBER_ARRAY_REGEX};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    prelude::rust_2015::Ok,
    sync::{Arc, RwLock}
};

pub trait IniSectionTrait {
    fn get_string(&self, key: &str) -> String;
    fn get_number(&self, key: &str) -> f32;
    fn get_number_default(&self, key: &str, default: f32) -> f32;
}
impl IniSectionTrait for RwLock<IniSection> {
    fn get_string(&self, key: &str) -> String {
        self.read().unwrap().get_string(key)
    }

    fn get_number(&self, key: &str) -> f32 {
        self.read().unwrap().get_number(key)
    }

    fn get_number_default(&self, key: &str, default: f32) -> f32 {
        self.read().unwrap().get_number_default(key, default)
    }
}

#[derive(Debug)]
pub struct IniSection {
    pub entries: Map<String, Value>,
    pub name:    String
}

impl IniSection {
    pub fn new(map: Value, name: String) -> Result<Self> {
        Ok(IniSection {
            entries: get_value_map(map)?,
            name
        })
    }

    pub fn new_empty(name: String) -> Self {
        IniSection {
            entries: Map::<String, Value>::with_capacity(10),
            name
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.entries.insert(key, Value::from(val));
    }

    pub fn get(&self, key: &str) -> Result<&Value> {
        Ok(self.entries.get(key).ok_or(anyhow!("option is none"))?)
    }

    pub fn has(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn get_string_option(&self, key: &str) -> Option<String> {
        match self.get_string_result(key) {
            Ok(val) => Some(val),
            Err(err) => {
                warn!("{:?}", err);
                None
            }
        }
    }

    pub fn get_string_result(&self, key: &str) -> Result<String> {
        if self.has(key) {
            Ok(self
                .get(key)?
                .as_str()
                .ok_or(anyhow!("option is none"))?
                .to_string())
        } else {
            bail!("value is none for key: {}", key)
        }
    }

    pub fn get_number_i32_from_str_option(&self, key: &str) -> Option<i32> {
        if let Some(val) = self.get_string_option(key) {
            let Ok(rs) = val.parse::<i32>() else {
                return None;
            };
            Some(rs)
        } else {
            None
        }
    }

    pub fn get_number_f32_from_str_option(&self, key: &str) -> Option<f32> {
        if let Some(val) = self.get_string_option(key) {
            let Ok(res) = val.parse::<f32>() else {
                error!("parse f64 fail: key={:?} {}", key, val);
                return None;
            };
            Some(res)
        } else {
            None
        }
    }

    pub fn get_number_i32_from_str(&self, key: &str) -> i32 {
        if let Some(val) = self.get_number_i32_from_str_option(key) {
            val
        } else {
            error!("get_number_i32_from_str no key={}", key);
            0i32
        }
    }

    pub fn get_number_i32_from_str_default(&self, key: &str, default: i32) -> i32 {
        if let Some(val) = self.get_number_i32_from_str_option(key) {
            val
        } else {
            default
        }
    }

    pub fn get_number_f32_from_str(&self, key: &str) -> f32 {
        if let Some(val) = self.get_number_f32_from_str_option(key) {
            val
        } else {
            error!("no key={}", key);
            0.0
        }
    }

    pub fn get_number_f64_from_str(&self, key: &str) -> f64 {
        if let Some(val) = self.get_number_f32_from_str_option(key) {
            val as f64
        } else {
            error!("no key={}", key);
            0.0
        }
    }

    pub fn get_string(&self, key: &str) -> String {
        self.get_string_default(key, "")
    }

    pub fn get_string_default(&self, key: &str, default: &str) -> String {
        self.get_string_result(key).unwrap_or(default.to_string())
    }

    pub fn get_number_option(&self, key: &str) -> Option<f32> {
        if let Some(val) = self.entries.get(key) {
            if let Some(val) = val.as_f64() {
                Some(val as f32)
            } else if let Some(val) = val.as_str() {
                if let Ok(res) = val.parse::<f32>() {
                    Some(res)
                } else {
                    warn!("error key={:?}", key);
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_number_default(&self, key: &str, default: f32) -> f32 {
        if let Some(val) = self.get_number_option(key) {
            val
        } else {
            default
        }
    }

    pub fn get_number(&self, key: &str) -> f32 {
        if let Some(val) = self.get_number_option(key) {
            val
        } else {
            warn!("none key={}", key);
            0.0
        }
    }

    pub fn get_number_i32(&self, key: &str) -> i32 {
        if let Some(val) = self.get_number_i32_option(key) {
            val
        } else {
            warn!("none key={}", key);
            // 0i32
            0
        }
    }

    pub fn get_number_i32_option(&self, key: &str) -> Option<i32> {
        let res = self.entries.get(key);
        if let Some(res) = res {
            if let Some(tmp) = res.as_i64() {
                Some(tmp as i32)
            } else {
                let Some(tmp) = res.as_str() else {
                    return None;
                };

                let Ok(res) = tmp.parse::<i32>() else {
                    error!("parse fail: key={:?} {}", key, tmp);
                    return None;
                };
                Some(res)
            }
        } else {
            None
        }
    }

    pub fn get_number_i32_default(&self, key: &str, default: i32) -> i32 {
        if let Some(val) = self.get_number_i32_option(key) {
            val
        } else {
            default
        }
    }

    pub fn get_bool_or_default(&self, key: &str, default: bool) -> bool {
        if let Some(val) = self.get_bool_option(key) {
            val
        } else {
            default
        }
    }

    ///与js代码不一致
    pub fn get_bool_option(&self, key: &str) -> Option<bool> {
        if let Some(i) = self.entries.get(key) {
            if let Some(i) = i.as_str() {
                return match i {
                    "yes" => Some(true),
                    "1" => Some(true),
                    "true" => Some(true),
                    "on" => Some(true),
                    "no" => Some(false),
                    "0" => Some(false),
                    "false" => Some(false),
                    "off" => Some(false),
                    _ => None
                };
            }
        }
        None
    }

    ///与js代码不一致
    pub fn get_bool(&self, key: &str) -> bool {
        if let Some(val) = self.get_bool_option(key) {
            val
        } else {
            warn!("get_bool: {:?}", key);
            false
        }
    }

    pub fn get_array(&self, key: &str) -> Vec<String> {
        self.get_array_with_default(key, None, None)
    }

    pub fn get_array_with_default(
        &self,
        key: &str,
        regex: Option<&str>,
        default: Option<Vec<String>>
    ) -> Vec<String> {
        let _regex = regex.unwrap_or(DEFAULT_REGEX);
        let default = default.unwrap_or(Vec::new());
        if let Some(t) = self.entries.get(key) {
            let Some(_val) = t.as_str() else {
                return default;
            };
        }
        default
    }

    pub fn get_number_array_defalut(&self, key: &str) -> Vec<f32> {
        self.get_number_array(key, DEFAULT_REGEX)
            .unwrap_or(Vec::new())
    }

    pub fn get_number_array(&self, key: &str, regex: &str) -> Result<Vec<f32>> {
        let n = self.get_string_option(key);
        let mut res = Vec::new();
        if let Some(n) = n {
            let n = n.trim();
            let regex_tmp = regex::Regex::new(GET_NUMBER_ARRAY_REGEX)?;
            let rs = split_by_regex(n, regex)?;
            for r in rs {
                let t: f32 = if regex_tmp.is_match(r.as_str()) {
                    let r = r.replace("%", "");
                    r.parse::<f32>()? / 100f32
                } else {
                    r.parse::<f32>()?
                };
                res.push(t);
            }
        }
        Ok(res)
    }

    ///no_case: 大小写不敏感，true：不管大小写。默认为false
    pub fn get_enum(&self, e: &str, enums: impl GetEnum, no_case: bool) -> i32 {
        let s = self.get_string_option(e);
        if let Some(s) = s {
            let Some(val) = enums.get_num_by_str(s.as_str()) else {
                return if no_case {
                    enums
                        .get_num_by_lowercase_str(s.as_str())
                        .unwrap_or(enums.get_num())
                } else {
                    enums.get_num()
                };
            };
            val
        } else {
            return enums.get_num();
        }
    }

    pub fn get_enum_array_default(&self, t: &str, enums: impl GetEnum) -> Vec<i32> {
        self.get_enum_array(t, enums, DEFAULT_REGEX, Vec::with_capacity(20), false)
    }

    pub fn get_enum_array(
        &self,
        t: &str,
        enums: impl GetEnum,
        i: &str,
        n: Vec<i32>,
        no_case: bool
    ) -> Vec<i32> {
        let r = self.get_string_option(t);
        if let Some(r) = r {
            let mut res = Vec::<i32>::with_capacity(20);
            let vals = split_by_regex(r.as_str(), i).unwrap_or(Vec::new());
            for o in vals {
                let mut i = false;
                if no_case {
                    if let Some(e) = enums.get_num_by_lowercase_str(o.as_str()) {
                        res.push(e);
                        i = true;
                    }
                } else {
                    if let Some(e) = enums.get_num_by_str(o.as_str()) {
                        res.push(e);
                        i = true;
                    }
                }
                if i == false {
                    //todo warn
                }
            }
            return res;
        } else {
            return n;
        }
    }

    pub fn get_highest_numberic_index() {
        //todo
    }

    pub fn get_concatented_values() {
        //todo
    }
}
#[derive(Clone, Debug)]
pub struct IniFile {
    pub name:     String,
    pub sections: HashMap<String, Arc<IniSection>>
}

impl IniFile {
    pub fn new(name: String, val: Value, ini: Option<Ini>) -> Result<Self> {
        // let name = string_to_static_str(name);
        let secs = get_value_map(val)?;
        let mut sections = HashMap::<String, IniSection>::with_capacity(secs.len() + 10);
        for (key, val) in secs {
            sections.insert(key.clone(), IniSection::new(val, key)?);
        }
        //mergeWith
        if let Some(ini) = ini {
            let maps = ini.get_map().ok_or(anyhow!("none"))?;
            for (key, map) in maps {
                if !sections.contains_key(key.as_str()) {
                    sections.insert(key.clone(), IniSection::new_empty(key.clone()));
                };
                let tmp = sections
                    .get_mut(key.as_str())
                    .ok_or(anyhow!("sections not contail: {}", key))?;
                if is_value_array(key.as_str()) {
                    bail!("should not be array");
                } else {
                    for (key, val) in map {
                        let Some(val) = val else {
                            warn!("key {} is none", key);
                            continue;
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

    pub fn get_or_create_section(&mut self, key: &str) -> Arc<IniSection> {
        let Some(section) = self.sections.get(key) else {
            let tmp = Arc::new(IniSection::new_empty(key.to_string()));
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
            bail!("get_section fail: name={}, key={}", &self.name, key);
        }
    }

    pub fn get_section_option(&self, key: &str) -> Option<&Arc<IniSection>> {
        //rules.ini去掉了NAWAST "GARADR" CAIRSFGL "NAHPAD"
        // debug!("name={}, key={}", &self.name, key);
        self.sections.get(key)
    }

    pub fn get_ordered_sections(&self) -> &HashMap<String, Arc<IniSection>> {
        &self.sections
    }
}
