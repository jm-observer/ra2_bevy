pub mod vxl;
pub mod pub_use;
pub mod color;
pub mod config;

pub trait GetEnum {
    fn get_num_by_str(&self, key: &str) -> Option<i32>;
    fn get_num_by_lowercase_str(&self, key: &str) -> Option<i32>;
    fn get_num(&self) -> i32;
}
