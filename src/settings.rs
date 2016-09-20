use std::collections::HashMap;
use rustbox::Color;

pub enum Value {
    Color(Color),
    #[allow(dead_code)]
    Number(i32),
    #[allow(dead_code)]
    String(String),
}

pub struct Settings {
    settings: HashMap<String, Value>, // = HashMap::new();
}

impl Settings {
    pub fn new() -> Settings {
        let mut s = HashMap::new();

        // TODO this isn't being used.
        s.insert("color-linenumbers".to_string(), Value::Color(Color::Yellow));
        return Settings { settings: s };
    }

    pub fn get(&self, setting: &str) -> Option<&Value> {
        self.settings.get(&setting.to_string())
    }

    #[allow(dead_code)]
    pub fn set(&mut self, setting: &str, value: Value) {
        self.settings.insert(setting.to_string(), value);
    }
}
