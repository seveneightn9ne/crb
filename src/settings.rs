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
    pub lineNumColor: Color,
    pub insertSpaces: bool, // False => tab
    pub numSpacesPerTab: u8,
}

impl Settings {
    pub fn new() -> Settings {
        return Settings {
            lineNumColor: Color::Yellow,
            insertSpaces: true,
            numSpacesPerTab: 4,
        };
    }

    // pub fn get(&self, setting: &str) -> Option<&Value> {
    // self.settings.get(&setting.to_string())
    // }
    //
    // #[allow(dead_code)]
    // pub fn set(&mut self, setting: &str, value: Value) {
    // self.settings.insert(setting.to_string(), value);
    // }
}
