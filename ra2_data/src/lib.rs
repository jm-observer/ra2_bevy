pub mod color;
pub mod config;
pub mod coord;
pub mod pub_use;
pub mod tile;
pub mod vxl;

pub trait GetEnum {
    fn get_num_by_str(&self, key: &str) -> Option<i32>;
    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32>;
    fn get_num(&self) -> i32;
}
