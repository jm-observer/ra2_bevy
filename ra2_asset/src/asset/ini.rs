use anyhow::Result;
use bevy::reflect::TypeUuid;
use configparser::ini::Ini;
use ra2_data::ini::IniFile;

use serde_json::Value;
use std::ops::Deref;

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "b0298126-fc12-4767-8d0f-ddab3da2ebf8"]
pub struct IniAsset(pub IniFile);

impl IniAsset {
    pub fn new(name: String, val: Value, ini: Option<Ini>) -> Result<Self> {
        Ok(Self(IniFile::new(name, val, ini)?))
    }
}

impl Deref for IniAsset {
    type Target = IniFile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
