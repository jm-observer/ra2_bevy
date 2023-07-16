use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
/// 光标(本次是否移动，累计停留时长)。暂时只考虑鼠标移动产生的效果
pub struct Cursor(pub(crate) bool, pub(crate) Duration);

impl Default for Cursor {
    fn default() -> Self {
        Cursor(false, Duration::new(0, 0))
    }
}
