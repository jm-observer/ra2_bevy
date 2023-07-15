use bevy::{prelude::*, reflect::TypeUuid};
use ra2_data::map::MapFile;
use std::{ops::Deref, sync::Arc};

#[derive(TypeUuid, Clone, Resource)]
#[uuid = "2b9f3349-8a0f-4b37-bb76-82c0c48a3f3e"]
pub struct MapAsset(pub(crate) Arc<MapFile>);

impl Deref for MapAsset {
    type Target = Arc<MapFile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
