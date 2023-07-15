use crate::{
    ini::IniSection
};
use std::sync::Arc;

pub struct TagsReader;
impl TagsReader {
    pub fn read(sec: Arc<IniSection>) -> Vec<Arc<Tag>> {
        // for (key, val) in
        let mut res = Vec::<Arc<Tag>>::with_capacity(20);
        for (key, val) in sec.entries.iter() {
            let slices: Vec<&str> = val.as_str().unwrap().split(',').collect();
            if slices.len() < 3 {
                // console.warn(`Invalid tag ${n}=${s}. Skipping.`);
                continue;
            }
            let r: usize = slices[0].parse().unwrap();
            if r > TAGREPEATTYPE.len() {
                // console.warn(`Invalid repeat value ${r} for tag id ${n}. Skipping.`);
                continue;
            }
            let a = Tag {
                id:          key.to_string(),
                repeat_type: r as i32,
                name:        slices[1].to_string(),
                trigger_id:  slices[2].to_string()
            };
            res.push(Arc::new(a));
        }
        res
    }
}

pub struct Tag {
    pub id:          String,
    pub repeat_type: i32,
    pub name:        String,
    pub trigger_id:  String
}

const TAGREPEATTYPE: [&str; 3] = ["OnceAny", "OnceAll", "Repeat"];
