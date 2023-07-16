use crate::GetEnum;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum LandType {
    Clear    = 0,
    Road     = 1,
    Rock     = 2,
    Beach    = 3,
    Rough    = 4,
    Railroad = 5,
    Water    = 6,
    Wall     = 7,
    Tiberium = 8,
    Cliff    = 9
}

impl GetEnum for LandType {
    fn get_num(&self) -> i32 {
        *self as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Clear" => Some(0),
            "Road" => Some(1),
            "Rock" => Some(2),
            "Beach" => Some(3),
            "Rough" => Some(4),
            "Railroad" => Some(5),
            "Water" => Some(6),
            "Wall" => Some(7),
            "Tiberium" => Some(8),
            "Cliff" => Some(9),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "clear" => Some(0),
            "road" => Some(1),
            "rock" => Some(2),
            "beach" => Some(3),
            "rough" => Some(4),
            "railroad" => Some(5),
            "water" => Some(6),
            "wall" => Some(7),
            "tiberium" => Some(8),
            "cliff" => Some(9),
            _ => None
        }
    }
}

impl From<LandType> for String {
    fn from(item: LandType) -> Self {
        match item {
            LandType::Clear => "Clear".to_string(),
            LandType::Road => "Road".to_string(),
            LandType::Rock => "Rock".to_string(),
            LandType::Beach => "Beach".to_string(),
            LandType::Rough => "Rough".to_string(),
            LandType::Railroad => "Railroad".to_string(),
            LandType::Water => "Water".to_string(),
            LandType::Wall => "Wall".to_string(),
            LandType::Tiberium => "Tiberium".to_string(),
            LandType::Cliff => "Cliff".to_string()
        }
    }
}

impl LandType {
    pub fn get_land_type(item: TerrainType) -> Self {
        match item {
            TerrainType::Default => LandType::Clear,
            TerrainType::Railroad => LandType::Railroad,
            TerrainType::Rock1 => LandType::Rock,
            TerrainType::Rock2 => LandType::Rock,
            TerrainType::Water => LandType::Water,
            TerrainType::Shore => LandType::Beach,
            TerrainType::Pavement => LandType::Road,
            TerrainType::Dirt => LandType::Road,
            TerrainType::Clear => LandType::Clear,
            TerrainType::Rough => LandType::Rough,
            TerrainType::Cliff => LandType::Cliff
        }
    }

    pub fn get_land_type_rule_name() -> [(LandType, &'static str); 10] {
        [
            (LandType::Clear, "Clear"),
            (LandType::Road, "Road"),
            (LandType::Rock, "Rock"),
            (LandType::Beach, "Beach"),
            (LandType::Rough, "Rough"),
            (LandType::Railroad, "Railroad"),
            (LandType::Water, "Water"),
            (LandType::Wall, "Wall"),
            (LandType::Tiberium, "Tiberium"),
            (LandType::Cliff, "Rock")
        ]
    }
}

#[derive(FromPrimitive, Copy, Clone, Hash, Debug)]
pub enum TerrainType {
    Default  = 0,
    ///铁路
    Railroad = 6,
    Rock1    = 7,
    Rock2    = 8,
    Water    = 9,
    ///海岸
    Shore    = 10,
    ///人行道
    Pavement = 11,
    ///尘埃 污物 泥土 尘
    Dirt     = 12,
    Clear    = 13,
    ///高低不平的地面
    Rough    = 14,
    ///悬崖；绝壁
    Cliff    = 15
}

impl From<TerrainType> for String {
    fn from(item: TerrainType) -> Self {
        match item {
            TerrainType::Default => "Default".to_string(),
            TerrainType::Railroad => "Railroad".to_string(),
            TerrainType::Rock1 => "Rock1".to_string(),
            TerrainType::Rock2 => "Rock2".to_string(),
            TerrainType::Water => "Water".to_string(),
            TerrainType::Shore => "Shore".to_string(),
            TerrainType::Pavement => "Pavement".to_string(),
            TerrainType::Dirt => "Dirt".to_string(),
            TerrainType::Clear => "Clear".to_string(),
            TerrainType::Rough => "Rough".to_string(),
            TerrainType::Cliff => "Cliff".to_string()
        }
    }
}
#[derive(Copy, Clone)]
pub enum ArmorType {
    None     = 0,
    Flak     = 1,
    Plate    = 2,
    Light    = 3,
    Medium   = 4,
    Heavy    = 5,
    Wood     = 6,
    Steel    = 7,
    Concrete = 8,
    Special1 = 9,
    Special2 = 10
}

impl GetEnum for ArmorType {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "None" => Some(0),
            "Flak" => Some(1),
            "Plate" => Some(2),
            "Light" => Some(3),
            "Medium" => Some(4),
            "Heavy" => Some(5),
            "Wood" => Some(6),
            "Steel" => Some(7),
            "Concrete" => Some(8),
            "Special_1" => Some(9),
            "Special_2" => Some(10),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "none" => Some(0),
            "flak" => Some(1),
            "plate" => Some(2),
            "light" => Some(3),
            "medium" => Some(4),
            "heavy" => Some(5),
            "wood" => Some(6),
            "steel" => Some(7),
            "concrete" => Some(8),
            "special_1" => Some(9),
            "special_2" => Some(10),
            _ => None
        }
    }
}
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum LocomotorType {
    Statue   = 0,
    Aircraft = 1,
    Chrono   = 2,
    Hover    = 3,
    Infantry = 4,
    Jumpjet  = 5,
    Missile  = 6,
    Ship     = 7,
    Vehicle  = 8
}

impl LocomotorType {
    pub fn locomotor_types_by_cls_id(key: &str) -> Option<LocomotorType> {
        match key {
            "{4A582746-9839-11d1-B709-00A024DDAFD1}" => Some(LocomotorType::Aircraft),
            "{4A582747-9839-11d1-B709-00A024DDAFD1}" => Some(LocomotorType::Chrono),
            "{4A582742-9839-11d1-B709-00A024DDAFD1}" => Some(LocomotorType::Hover),
            "{4A582744-9839-11d1-B709-00A024DDAFD1}" => Some(LocomotorType::Infantry),
            "{92612C46-F71F-11d1-AC9F-006008055BB5}" => Some(LocomotorType::Jumpjet),
            "{B7B49766-E576-11d3-9BD9-00104B972FE8}" => Some(LocomotorType::Missile),
            "{2BEA74E1-7CCA-11d3-BE14-00104B62A16C}" => Some(LocomotorType::Ship),
            "{4A582741-9839-11d1-B709-00A024DDAFD1}" => Some(LocomotorType::Vehicle),
            _ => None
        }
    }

    pub fn default_speeds_by_locomotor(key: Self) -> Option<SpeedType> {
        match key {
            LocomotorType::Aircraft => Some(SpeedType::Wheel),
            LocomotorType::Hover => Some(SpeedType::Hover),
            LocomotorType::Infantry => Some(SpeedType::Foot),
            LocomotorType::Jumpjet => Some(SpeedType::Winged),
            LocomotorType::Missile => Some(SpeedType::Winged),
            LocomotorType::Ship => Some(SpeedType::Float),
            _ => None
        }
    }
}
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum SpeedType {
    Foot       = 0,
    Track      = 1,
    Wheel      = 2,
    Hover      = 3,
    Float      = 4,
    FloatBeach = 5,
    /// 两栖
    Amphibious = 6,
    /// 风行
    Winged     = 7
}
impl SpeedType {
    pub fn get_by_str(name: &str) -> Option<Self> {
        match name {
            "Foot" => Some(SpeedType::Foot),
            "Track" => Some(SpeedType::Track),
            "Wheel" => Some(SpeedType::Wheel),
            "Hover" => Some(SpeedType::Hover),
            "Float" => Some(SpeedType::Float),
            "FloatBeach" => Some(SpeedType::FloatBeach),
            "Amphibious" => Some(SpeedType::Amphibious),
            "Winged" => Some(SpeedType::Winged),
            _ => None
        }
    }
}

impl GetEnum for SpeedType {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Combat" => Some(0),
            "Tech" => Some(1),
            "Resource" => Some(2),
            "Power" => Some(3),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "combat" => Some(0),
            "tech" => Some(1),
            "resource" => Some(2),
            "power" => Some(3),
            _ => None
        }
    }
}
#[derive(Copy, Clone)]
pub enum MovementZone {
    Amphibious          = 0,
    AmphibiousCrusher   = 1,
    AmphibiousDestroyer = 2,
    Crusher             = 3,
    CrusherAll          = 4,
    Destroyer           = 5,
    Fly                 = 6,
    Infantry            = 7,
    InfantryDestroyer   = 8,
    Normal              = 9,
    Subterranean        = 10,
    Water               = 11
}
#[derive(Copy, Clone)]
pub enum PipColor {
    Green  = 0,
    Yellow = 1,
    White  = 2,
    Red    = 3,
    Blue   = 4
}

impl GetEnum for MovementZone {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Amphibious" => Some(0),
            "AmphibiousCrusher" => Some(1),
            "AmphibiousDestroyer" => Some(2),
            "Crusher" => Some(3),
            "CrusherAll" => Some(4),
            "Destroyer" => Some(5),
            "Fly" => Some(6),
            "Infantry" => Some(7),
            "InfantryDestroyer" => Some(8),
            "Normal" => Some(9),
            "Subterranean" => Some(10),
            "Water" => Some(11),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "amphibious" => Some(0),
            "amphibiouscrusher" => Some(1),
            "amphibiousdestroyer" => Some(2),
            "crusher" => Some(3),
            "crusherall" => Some(4),
            "destroyer" => Some(5),
            "fly" => Some(6),
            "infantry" => Some(7),
            "infantrydestroyer" => Some(8),
            "normal" => Some(9),
            "subterranean" => Some(10),
            "water" => Some(11),
            _ => None
        }
    }
}

impl GetEnum for PipColor {
    fn get_num(&self) -> i32 {
        *self as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Green" => Some(0),
            "Yellow" => Some(1),
            "White" => Some(2),
            "Red" => Some(3),
            "Blue" => Some(4),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "green" => Some(0),
            "yellow" => Some(1),
            "white" => Some(2),
            "red" => Some(3),
            "blue" => Some(4),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
///建筑类别？
pub enum BuildCat {
    ///武器建筑？
    Combat   = 0,
    ///技术？
    Tech     = 1,
    ///资源？
    Resource = 2,
    ///电力
    Power    = 3
}

impl GetEnum for BuildCat {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Combat" => Some(0),
            "Tech" => Some(1),
            "Resource" => Some(2),
            "Power" => Some(3),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "combat" => Some(0),
            "tech" => Some(1),
            "resource" => Some(2),
            "power" => Some(3),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
pub enum FactoryType {
    None          = 0,
    BuildingType  = 1,
    InfantryType  = 2,
    UnitType      = 3,
    NavalUnitType = 4,
    AircraftType  = 5
}
impl GetEnum for FactoryType {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "None" => Some(0),
            "BuildingType" => Some(1),
            "InfantryType" => Some(2),
            "UnitType" => Some(3),
            "NavalUnitType" => Some(3),
            "AircraftType" => Some(3),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "none" => Some(0),
            "buildingtype" => Some(1),
            "infantrytype" => Some(2),
            "unittype" => Some(3),
            "navalunittype" => Some(3),
            "aircrafttype" => Some(3),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
pub enum VeteranAbility {
    FASTER    = 0,
    STRONGER  = 1,
    FIREPOWER = 2,
    SCATTER   = 3,
    ROF       = 4,
    SIGHT     = 5,
    SELFHEAL  = 6
}

impl GetEnum for VeteranAbility {
    fn get_num(&self) -> i32 {
        self.clone() as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "FASTER" => Some(0),
            "STRONGER" => Some(1),
            "FIREPOWER" => Some(2),
            "SCATTER" => Some(3),
            "ROF" => Some(4),
            "SIGHT" => Some(5),
            "SELF_HEAL" => Some(6),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "faster" => Some(0),
            "stronger" => Some(1),
            "firepower" => Some(2),
            "scatter" => Some(3),
            "rof" => Some(4),
            "sight" => Some(5),
            "self_heal" => Some(6),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum WeaponType {
    Primary     = 0,
    Secondary   = 1,
    DeathWeapon = 2
}

impl GetEnum for WeaponType {
    fn get_num(&self) -> i32 {
        *self as i32
    }

    fn get_num_by_str(&self, key: &str) -> Option<i32> {
        match key {
            "Primary" => Some(0),
            "Secondary" => Some(1),
            "DeathWeapon" => Some(2),
            _ => None
        }
    }

    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32> {
        let key = key.to_lowercase();
        let key = key.as_str();
        match key {
            "primary" => Some(0),
            "secondary" => Some(1),
            "deathweapon" => Some(2),
            _ => None
        }
    }
}

pub enum InfDeathType {
    None        = 0,
    Gunfire     = 1,
    Explode     = 2,
    ExplodeAlt  = 3,
    Fire        = 4,
    Electro     = 5,
    HeadExplode = 6,
    Nuke        = 7
}
