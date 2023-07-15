mod res_loader;

use rand::*;
// use regex::Regex;
use anyhow::{bail, Result};
use bevy::prelude::debug;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

pub const ASSETS_INIS_PATH: &str = "./assets/inis/";
pub const ASSETS_PATH: &str = "./assets/ra2/";
pub const DEFAULT_REGEX: &str = ",\\s*";
pub const GET_NUMBER_ARRAY_REGEX: &str = "%$";

pub fn get_value_map(val: Value) -> anyhow::Result<Map<String, Value>> {
    match val {
        Value::Object(map) => Ok(map),
        _ => {
            bail!("get_value_map error");
        }
    }
}

pub fn random_usize(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

pub fn read_value_from_file<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

pub fn read_common_from_file<T>(path: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de> {
    debug!("path={}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn pad(num: i32, len: i32) -> String {
    let num = num.to_string();
    let num_len = num.len() as i32;
    let len = len - num_len;
    let mut res = "".to_string();
    for _ in 0..len {
        res.push('0');
    }
    res.push_str(num.as_str());
    res
}

pub fn as_number(val: &str) -> Result<f64> {
    Ok(val.parse::<f64>()?)
}
pub fn as_bool(val: f64) -> bool {
    if val > 0f64 { true } else { false }
}

pub fn split_by_default(text: &str) -> Result<Vec<String>> {
    split_by_regex(text, DEFAULT_REGEX)
}
pub fn split_by_regex(text: &str, regex: &str) -> Result<Vec<String>> {
    let regex = regex::Regex::new(regex)?;
    Ok(regex
        .split(text)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>())
}
//if "GNCY".index(chars.next()?) == -1 ||
// "ATU".index(chars.next()?) == -1 {

pub fn need_change_name(name: &str) -> Result<bool> {
    let regex = regex::Regex::new("^[GNCY][ATU].*")?;
    Ok(regex.is_match(name))
}

pub fn is_between(test: i32, min: i32, max: i32) -> bool {
    if test >= min && test <= max {
        true
    } else {
        false
    }
}

// #[test]
// fn test_need_change_name() {
//     assert!(need_change_name("GT01"));
//     assert!(!need_change_name("G01"));
//     assert!(!need_change_name("1GT01"));
//     assert!(need_change_name("CU01"));
// }

// #[test]
// fn test_split_by_default() {
//     debug!("{:?}", split_by_default("32, u6,4,  io"));
// }

// pub fn create_gba_png(
//     path: &str,
//     datas: &[u8],
//     width: u32,
//     height: u32
// ) {
//     let path = Path::new(path);
//     let file = File::create(path)?;
//     let ref mut w = BufWriter::new(file);
//     let mut encoder = png::Encoder::new(w, width, height); // Width
// is 2 pixels and height is 1.     encoder.
// set_color(png::ColorType::Rgb);     encoder.
// set_depth(png::BitDepth::Eight);     let mut writer =
// encoder.write_header()?;     debug!("{:?}",
// writer.write_image_data(datas)); }
// pub fn create_rgba_png(
//     path: &str,
//     datas: &[u8],
//     width: u32,
//     height: u32
// ) {
//     let path = Path::new(path);
//     let file = File::create(path)?;
//     let ref mut w = BufWriter::new(file);
//     let mut encoder = png::Encoder::new(w, width, height); // Width
// is 2 pixels and height is 1.     encoder.
// set_color(png::ColorType::Rgba);     encoder.
// set_depth(png::BitDepth::Eight);     let mut writer =
// encoder.write_header()?;     debug!("{:?}",
// writer.write_image_data(datas)); }

pub fn is_true(option: Option<bool>) -> bool {
    if let Some(val) = option { val } else { false }
}

pub fn copy_hash_arc<T: Clone>(source: &HashMap<String, T>) -> HashMap<String, T> {
    let mut dis = HashMap::<String, T>::with_capacity(source.len());
    for (key, val) in source {
        dis.insert(key.clone(), val.clone());
    }
    dis
}

pub fn rect_contains_point(
    e_x: i32,
    e_y: i32,
    e_width: i32,
    e_height: i32,
    t_x: i32,
    t_y: i32
) -> bool {
    t_x >= e_x && t_x <= e_x + e_width && t_y >= e_y && t_y <= e_y + e_height
}
