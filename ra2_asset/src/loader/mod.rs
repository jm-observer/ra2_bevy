mod palette;
mod ini;

pub use palette::*;
pub use ini::*;

use anyhow::{anyhow, Result};
use bevy::asset::LoadContext;


fn get_file_name(load_context: &LoadContext) -> Result<String> {
    Ok(load_context
        .path()
        .file_name()
        .ok_or(anyhow!("file_name is none"))?
        .to_str()
        .ok_or(anyhow!("file_name is none"))?
        .to_string())
}
