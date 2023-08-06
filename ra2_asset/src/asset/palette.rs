use crate::color::RaColor;
use anyhow::{bail, Result};
use bevy::{log::error, reflect::TypeUuid};
use serde_json::Value;

#[derive(Clone, Debug, TypeUuid, Default)]
#[uuid = "3c047c2f-2173-44f5-bdfd-c3087c8d89e2"]
pub struct PaletteAsset {
    //调色板
    pub datas: Vec<u8> //[Color; 256],
}

impl PaletteAsset {
    pub fn new_from_array(val: &[u8]) -> Self {
        PaletteAsset {
            datas: val.to_vec()
        }
    }
}
