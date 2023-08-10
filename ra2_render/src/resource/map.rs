use crate::data::map::TileCollection;
use bevy::ecs::system::Resource;
use ra2_asset::asset::MapAsset;
use ra2_data::{
    color::RaColor,
    rule::{
        AircraftRule, BuildingRule, CountryRule, GeneralRules, InfantryRule, LandRule, OverlayRule,
        TiberiumRule, VehicleRule, WarheadRule
    },
    ty::LandType
};
use std::{collections::HashMap, ops::Deref, sync::Arc};

#[derive(Resource)]
pub struct MapRes {
    pub res: MapAsset
}

#[derive(Resource)]
pub struct TileCollectionRes {
    tcr: Arc<TileCollection>
}
impl TileCollectionRes {
    pub fn from(tcr: Arc<TileCollection>) -> Self {
        println!("TileCollectionRes 初始化完毕");
        Self { tcr }
    }

    pub fn get_tile_collection(&self) -> Arc<TileCollection> {
        self.tcr.clone()
    }
}
impl Deref for TileCollectionRes {
    type Target = Arc<TileCollection>;

    fn deref(&self) -> &Self::Target {
        &self.tcr
    }
}

#[derive(Resource)]
pub struct GeneralRuleRes(pub Arc<GeneralRules>);

#[derive(Resource, Debug)]
pub struct CountryRuleRes(pub HashMap<&'static str, Arc<CountryRule>>);
#[derive(Resource, Debug)]
pub struct PlayerColorRes(pub HashMap<&'static str, Arc<RaColor>>);

impl From<HashMap<&'static str, Arc<RaColor>>> for PlayerColorRes {
    fn from(value: HashMap<&'static str, Arc<RaColor>>) -> Self {
        PlayerColorRes(value)
    }
}

#[derive(Resource)]
pub struct BuildingRuleRes(pub HashMap<String, Arc<BuildingRule>>);
#[derive(Resource)]
pub struct InfantryRuleRes(pub HashMap<String, Arc<InfantryRule>>);
#[derive(Resource)]
pub struct VehicleRuleRes(pub HashMap<String, Arc<VehicleRule>>);
#[derive(Resource)]
pub struct AircraftRuleRes(pub HashMap<String, Arc<AircraftRule>>);
#[derive(Resource)]
pub struct LandRuleRes(pub HashMap<LandType, Arc<LandRule>>);
#[derive(Resource)]
pub struct OverlayRuleRes(pub HashMap<i32, Arc<OverlayRule>>);
#[derive(Resource)]
pub struct TiberiumRuleRes(pub Vec<Arc<TiberiumRule>>);
#[derive(Resource)]
pub struct WarheadRuleRes(pub Vec<Arc<WarheadRule>>);
