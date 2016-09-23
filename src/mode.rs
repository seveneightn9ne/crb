use rustbox::Key;
use std::collections::HashMap;
use state;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Clone)]
pub enum Command {
    MoveLeft(i32),
    MoveRight(i32),
    MoveUp(i32),
    MoveDown(i32),
    Scroll(i32),
    Quit,
    Insert(char),
    Delete(Direction),
    NewLine,
    Unknown,
    ChangeMode(Mode),
    RecompileSelf,
    Save,
    Digit(u32),
    FocusWindow(u32),
}

#[derive(Debug, Clone)]
pub enum Direction {
    F,
    B,
}

type ModeMap = HashMap<Key, Command>;

pub fn map(mode: Mode, key: Key, state: &mut state::State) -> Command {
    match mode {
        Mode::Insert => {
            match key {
                Key::Char(c) => Command::Insert(c),
                Key::Tab => Command::Insert('\t'),
                Key::Esc => Command::ChangeMode(Mode::Normal),
                Key::F(1) => Command::ChangeMode(Mode::Normal),
                Key::Backspace => Command::Delete(Direction::B),
                Key::Enter => Command::NewLine,
                Key::Down => Command::MoveDown(1),
                Key::Up => Command::MoveUp(1),
                Key::Left => Command::MoveLeft(1),
                Key::Right => Command::MoveRight(1),
                _ => Command::Unknown,
            }
        }
        Mode::Normal => {
            match key {
                Key::Char('q') => Command::Quit,
                Key::Char('j') => Command::MoveDown(state.num_prefix as i32),
                Key::Char('k') => Command::MoveUp(state.num_prefix as i32),
                Key::Char('h') => Command::MoveLeft(state.num_prefix as i32),
                Key::Char('l') => Command::MoveRight(state.num_prefix as i32),
                Key::Char('x') => Command::Delete(Direction::F),
                Key::Down => Command::MoveDown(state.num_prefix as i32),
                Key::Up => Command::MoveUp(state.num_prefix as i32),
                Key::Left => Command::MoveLeft(state.num_prefix as i32),
                Key::Right => Command::MoveRight(state.num_prefix as i32),
                Key::Char('i') => Command::ChangeMode(Mode::Insert),
                Key::Char('b') => Command::Scroll(state.num_prefix as i32),
                Key::Char('v') => Command::Scroll(-(state.num_prefix as i32)),
                Key::Char('r') => Command::RecompileSelf,
                Key::Char(' ') => Command::Save,
                Key::Char(d) if d.is_digit(10) => Command::Digit(d.to_digit(10).unwrap()),
                Key::Char('w') => Command::FocusWindow(state.num_prefix),
                _ => Command::Unknown,
            }
        }
    }
}
