use crate::{
    ini::IniSection,
    map::tag::Tag,
    pub_use::*
};
use ra2_util::{as_bool, as_number};
use std::{collections::HashMap, sync::Arc};

///

//触发器 触发
pub enum TriggerEventType {
    AnyEvent = 8
}
pub struct TriggerEvent;
pub struct TriggerAction;
pub enum TriggerActionType {
    PlaySoundFxAt = 99
}
pub struct Trigger;

pub struct TriggerReader;
impl TriggerReader {
    //read(e, t, i, n)
    pub fn read(
        e: Arc<IniSection>,
        t: Arc<IniSection>,
        i: Arc<IniSection>,
        n: &Vec<Arc<Tag>>
    ) -> Result<Vec<TriggerUnit>> {
        let _s = Self::read_triggers(e, n)?;
        let _r = Self::read_events(t);
        let _a = Self::read_actions(i);
        //todo
        // panic!("");
        Ok(Vec::with_capacity(30))
    }

    //readTriggers(e, t)(e, t, i, n)
    pub fn read_triggers(e: Arc<IniSection>, n: &Vec<Arc<Tag>>) -> Result<Vec<TriggerUnit>> {
        let mut tu_vec = Vec::<TriggerUnit>::with_capacity(20);
        let mut iter = n.into_iter();
        for (key, val) in e.entries.iter() {
            let slices: Vec<&str> = val.as_str().unwrap().split(',').collect();
            if slices.len() < 8 {
                //todo warn
                continue;
            }
            let res = iter.find(|x| x.trigger_id.eq(key));
            if res.is_none() {
                //todo warn
                continue;
            }
            let attached_trigger_id = if "<none>" == slices[1] {
                None
            } else {
                Some(slices[1].to_string())
            };
            let tu = TriggerUnit {
                id: key.clone(),
                house_name: slices[0].to_string(),
                attached_trigger_id,
                name: slices[2].to_string(),
                disabled: as_bool(as_number(slices[3])?),
                difficulties: Difficulty {
                    easy:   as_bool(as_number(slices[4])?),
                    medium: as_bool(as_number(slices[5])?),
                    hard:   as_bool(as_number(slices[6])?)
                },
                events: Vec::<String>::with_capacity(20),
                actions: Vec::<String>::with_capacity(20),
                tag: res.unwrap().clone()
            };
            tu_vec.push(tu);
        }
        Ok(tu_vec)
    }

    pub fn read_events(_t: Arc<IniSection>) -> HashMap<String, TriggerEvent> {
        // panic!("");
        //todo
        HashMap::with_capacity(50)
    }

    pub fn read_actions(_t: Arc<IniSection>) -> HashMap<String, TriggerAction> {
        //todo
        HashMap::with_capacity(50)
    }
}

pub struct TriggerUnit {
    id:                  String,
    house_name:          String,
    attached_trigger_id: Option<String>,
    name:                String,
    disabled:            bool,
    difficulties:        Difficulty,
    events:              Vec<String>,
    actions:             Vec<String>,
    tag:                 Arc<Tag>
}
pub struct Difficulty {
    easy:   bool,
    medium: bool,
    hard:   bool
}
