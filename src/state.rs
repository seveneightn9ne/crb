use settings::Settings;
use std::sync::Mutex;

pub struct State {
    pub settings: Settings,
    pub next_window_index: i32,
    pub num_prefix: u32,
    pub typing_num_prefix: bool,
}

impl State {
    pub fn new() -> State {
        return State {
            settings: Settings::new(),
            next_window_index: 1,
            num_prefix: 1,
            typing_num_prefix: false,
        };
    }

    pub fn type_num_prefix(&mut self, d: u32) {
        if self.typing_num_prefix {
            self.num_prefix = self.num_prefix * 10 + d;
        } else {
            self.typing_num_prefix = true;
            self.num_prefix = d;
        }
    }

    pub fn end_num_prefix(&mut self) {
        self.typing_num_prefix = false;
        self.num_prefix = 1;
    }
}

pub fn do_safe<F, T>(statelock: &Mutex<State>, func: F) -> T
    where F: Fn(&mut State) -> T
{
    let mut state = statelock.lock().unwrap();
    let t = func(&mut *state);
    t
}
