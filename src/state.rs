use settings::Settings;

pub struct State {
    pub settings: Settings,
    pub next_window_index: i32,
}

impl State {
    pub fn new() -> State {
        return State {
            settings: Settings::new(),
            next_window_index: 1,
        };
    }
}
