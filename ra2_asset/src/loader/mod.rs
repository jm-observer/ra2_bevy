pub mod palette;
use anyhow::{anyhow, Result};
use bevy::asset::LoadContext;

mod ini;

fn get_file_name(load_context: &LoadContext) -> Result<String> {
    Ok(load_context
        .path()
        .file_name()
        .ok_or(anyhow!("file_name is none"))?
        .to_str()
        .ok_or(anyhow!("file_name is none"))?
        .to_string())
}
