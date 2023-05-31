use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Hash, Resource)]
pub struct RaColor {
    r: u8,
    g: u8,
    b: u8
}

const T: [f32; 16] = [
    63.0, 59.0, 55.0, 52.0, 48.0, 44.0, 41.0, 37.0, 33.0, 30.0, 26.0, 22.0, 19.0, 15.0, 11.0, 8.0
];

impl RaColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RaColor { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        RaColor::new(r, g, b)
    }

    pub fn from_hsv(t: f32, i: f32, n: f32) -> Self {
        let s: f32;
        let r: f32;
        let a: f32;
        let t = t / 255f32 * 360f32 % 360f32;
        let n = n / 255f32;
        let i = i / 255f32;
        if i == 0f32 {
            s = n;
            r = n;
            a = n;
        } else {
            let u = t / 60f32;
            let h = u as i32;
            //math.floor(u);
            let c = u - h as f32;
            let e = n * (1f32 - i);
            let o = n * (1f32 - i * c);
            let l = n * (1f32 - i * (1f32 - c));
            match h {
                0 => {
                    s = n;
                    r = l;
                    a = e;
                },
                1 => {
                    s = o;
                    r = n;
                    a = e;
                },
                2 => {
                    s = e;
                    r = n;
                    a = l;
                },
                3 => {
                    s = e;
                    r = o;
                    a = n;
                },
                4 => {
                    s = l;
                    r = e;
                    a = n;
                },
                5 => {
                    s = n;
                    r = e;
                    a = o;
                },
                _ => {
                    panic!()
                }
            }
        }
        Self::from_rgb((255f32 * s) as u8, (255f32 * r) as u8, (255f32 * a) as u8)
    }

    pub fn as_hex(&self) {
        panic!("");
    }

    pub fn as_hex_string(&self) {
        panic!("");
    }

    pub fn clone(&self) {
        panic!("");
    }

    pub fn remap(&mut self, color: &RaColor, index: usize) {
        self.r = (color.r as f32 / 255.0 * T[index - 16] * 4.0) as u8;
        self.g = (color.g as f32 / 255.0 * T[index - 16] * 4.0) as u8;
        self.b = (color.b as f32 / 255.0 * T[index - 16] * 4.0) as u8;
    }
}
