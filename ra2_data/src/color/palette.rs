use crate::color::RaColor;
use anyhow::{bail, Result};
use bevy::{log::error, reflect::TypeUuid};
use serde_json::Value;

#[derive(Clone, Debug, TypeUuid, Default)]
#[uuid = "3c047c2f-2173-44f5-bdfd-c3087c8d89e2"]
pub struct Palette {
    //调色板
    pub colors: Vec<RaColor> //[Color; 256],
}

impl Palette {
    pub fn new(val: Value) -> Result<Self> {
        if val.is_array() {
            let val_array = val.as_array().unwrap().as_slice();
            let mut index: usize = 0;
            let len = val_array.len() / 3 as usize;
            let mut colors = Vec::<RaColor>::with_capacity(256);
            while index < len {
                colors.push(RaColor::from_rgb(
                    4 * val_array[3 * index].as_i64().unwrap() as u8,
                    4 * val_array[3 * index + 1].as_i64().unwrap() as u8,
                    4 * val_array[3 * index + 2].as_i64().unwrap() as u8
                ));
                index += 1;
            }
            Ok(Palette { colors })
        } else {
            bail!("Palette should be array");
        }
    }

    pub fn new_from_array(val: &[u8]) -> Self {
        let len = val.len() / 3 as usize;
        let mut colors = Vec::<RaColor>::with_capacity(256);
        let mut index = 0;
        while index < len {
            colors.push(RaColor::from_rgb(
                4 * val[3 * index],
                4 * val[3 * index + 1],
                4 * val[3 * index + 2]
            ));
            index += 1;
        }
        Palette { colors }
    }

    pub fn get_all_color(&self) -> &Vec<RaColor> {
        &self.colors
    }

    pub fn get_color(&self, index: usize) -> &RaColor {
        &self.colors[index]
    }

    pub fn get_color_as_hex(&self, index: usize) {
        self.get_color(index).as_hex()
    }

    pub fn get_hash(&self) {
        error!("todo");
    }

    pub fn remap(&mut self, e: &RaColor) {
        for i in 16..32 {
            self.colors[i].remap(e, i);
        }
    }

    pub fn size(&self) -> usize {
        self.colors.len()
    }
}
