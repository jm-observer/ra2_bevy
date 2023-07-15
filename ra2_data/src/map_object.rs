use crate::{map::*, overlay::Overlay, structure::Structure};

pub struct MapObject {
    ///?
    pub id:        i32,
    pub name:      String,
    pub owner:     Option<String>,
    pub types:     ObjectType,
    pub rx:        i32,
    pub ry:        i32,
    pub on_bridge: bool,
    pub terrain:   Option<Terrain>,
    pub infantry:  Option<Infantry>,
    pub vehicle:   Option<Vehicle>,
    pub aircraft:  Option<Aircraft>,
    pub overlay:   Option<Overlay>,
    pub smudge:    Option<Smudge>,
    pub structure: Option<Structure>,
    pub health:    Option<String>
}

impl MapObject {
    pub fn new(name: String, types: ObjectType, rx: i32, ry: i32, owner: Option<String>) -> Self {
        MapObject {
            ///?
            id: 0i32,
            name,
            types,
            rx,
            ry,
            on_bridge: false,
            terrain: None,
            infantry: None,
            vehicle: None,
            aircraft: None,
            overlay: None,
            smudge: None,
            structure: None,
            owner,
            health: None
        }
    }

    pub fn is_structure(&self) -> bool {
        matches!(self.types, ObjectType::Building)
    }

    pub fn is_vehicle(&self) -> bool {
        matches!(self.types, ObjectType::Vehicle)
    }

    pub fn is_infantry(&self) -> bool {
        matches!(self.types, ObjectType::Infantry)
    }

    pub fn is_aircraft(&self) -> bool {
        matches!(self.types, ObjectType::Aircraft)
    }

    pub fn is_terrain(&self) -> bool {
        matches!(self.types, ObjectType::Terrain)
    }

    pub fn is_smudge(&self) -> bool {
        matches!(self.types, ObjectType::Smudge)
    }

    pub fn is_overlay(&self) -> bool {
        matches!(self.types, ObjectType::Overlay)
    }

    pub fn is_ownable(&self) -> bool {
        self.owner.is_some()
    }

    pub fn is_named(&self) -> bool {
        panic!()
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ObjectType {
    None       = 0,
    Aircraft   = 1,
    Building   = 2,
    Infantry   = 3,
    Overlay    = 4,
    Smudge     = 5,
    Terrain    = 6,
    Vehicle    = 7,
    Animation  = 8,
    Projectile = 9
}

impl ObjectType {
    pub fn is_techno(&self) -> bool {
        //TechnoRule
        match self {
            ObjectType::Aircraft
            | ObjectType::Building
            | ObjectType::Vehicle
            | ObjectType::Infantry => true,
            _ => false
        }
    }

    pub fn is_smudge(&self) -> bool {
        matches!(self, ObjectType::Smudge)
    }
}
