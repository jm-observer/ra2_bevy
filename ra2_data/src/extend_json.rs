use serde_json::Value;

pub trait JsonExtend {
    fn get_i32(&self, name: &str) -> i32;

    fn get_i32_or_default(&self, name: &str, default: i32) -> i32;

    fn get_bool(&self, name: &str) -> bool;

    fn get_string(&self, name: &str) -> String;

    fn get_str(&self, name: &str) -> &str;

    fn get_array(&self, name: &str) -> Vec<Value>;
}

impl JsonExtend for Value {
    fn get_i32(&self, name: &str) -> i32 {
        self.get(name).unwrap().as_i64().unwrap() as i32
    }

    fn get_i32_or_default(&self, name: &str, default: i32) -> i32 {
        let val = self.get(name);
        if let Some(val) = val {
            let val = val.as_i64();
            if val.is_none() {
                return default;
            } else {
                val.unwrap() as i32
            }
        } else {
            default
        }
    }

    fn get_bool(&self, name: &str) -> bool {
        self.get(name).unwrap().as_bool().unwrap()
    }

    fn get_string(&self, name: &str) -> String {
        self.get(name).unwrap().as_str().unwrap().to_string()
    }

    fn get_str(&self, name: &str) -> &str {
        self.get(name).unwrap().as_str().unwrap()
    }

    fn get_array(&self, name: &str) -> Vec<Value> {
        self.get(name).unwrap().as_array().unwrap().clone()
    }
}
