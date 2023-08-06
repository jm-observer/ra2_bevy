use crate::{ini::IniSection, pub_use::Vec3};
use std::sync::Arc;

///照明 灯光 采光 打光
#[derive(Copy, Clone)]
pub struct Lighting {
    pub level:      f64,
    pub ambient:    f64,
    pub red:        f64,
    pub green:      f64,
    pub blue:       f64,
    pub ground:     f64,
    /// ?
    pub force_tint: bool
}
impl Lighting {
    pub fn init() -> Self {
        Lighting {
            level:      0.032f64,
            ambient:    1f64,
            red:        1f64,
            green:      1f64,
            blue:       1f64,
            ground:     0f64,
            force_tint: false
        }
    }

    pub fn read_default(&mut self, sec: Arc<IniSection>) {
        self.read(sec, "");
    }

    pub fn read(&mut self, sec: Arc<IniSection>, pre: &str) {
        let pre = pre.to_string();
        let mut tmp = pre.clone();
        tmp.push_str("level");
        self.level = sec.get_number_default(tmp.as_str(), 0.032);
        tmp = pre.clone();
        tmp.push_str("ambient");
        self.ambient = sec.get_number_default(tmp.as_str(), 1.0);
        tmp = pre.clone();
        tmp.push_str("red");
        self.red = sec.get_number_default(tmp.as_str(), 1.0);
        tmp = pre.clone();
        tmp.push_str("green");
        self.green = sec.get_number_default(tmp.as_str(), 1.0);
        tmp = pre.clone();
        tmp.push_str("blue");
        self.blue = sec.get_number_default(tmp.as_str(), 1.0);
        tmp = pre.clone();
        tmp.push_str("ground");
        self.ground = sec.get_number_default(tmp.as_str(), 0.0);
    }

    pub fn compute(&self, lighting_type: LightingType, _z: i32) -> Vec3 {
        match lighting_type {
            LightingType::None => Vec3::new(1.0, 1.0, 1.0),
            _ => {
                let res = self.compute_tint(lighting_type);
                // res.multiplyScalar(self.ambient + self.ground +
                // self.compute_level(lighting_type, z as f64));
                res
            }
        }
    }

    pub fn compute_default(&self, lighting_type: LightingType) -> Vec3 {
        self.compute(lighting_type, 0)
    }

    pub fn compute_level(&self, lighting_type: LightingType, z: f64) -> f64 {
        if lighting_type as i32 >= 2 {
            self.level * (z - 1f64)
        } else {
            0f64
        }
    }

    pub fn compute_tint(&self, lighting_type: LightingType) -> Vec3 {
        if lighting_type as i32 >= 4 || self.force_tint {
            Vec3::new(self.red as f32, self.green as f32, self.blue as f32)
        } else {
            Vec3::new(1.0, 1.0, 1.0)
        }
    }

    pub fn get_ambient_intensity(&self) -> f64 {
        self.ambient + self.ground
    }
}
#[derive(Eq, PartialEq, Debug)]
pub enum LightingType {
    None    = 0,
    Global  = 1,
    Level   = 2,
    //周围的；外界的；环绕的
    Ambient = 3,
    Full    = 4,
    Default = 5
}
