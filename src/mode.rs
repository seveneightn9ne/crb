use rustbox::Key;
use std::collections::HashMap;

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
}

#[derive(Debug, Clone)]
pub enum Direction {
    F,
    B,
}

type ModeMap = HashMap<Key, Command>;

pub fn map(mode: Mode, key: Key) -> Command {
    match mode {
        Mode::Insert => {
            match key {
                Key::Char(c) => Command::Insert(c),
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
                Key::Char('j') => Command::MoveDown(1),
                Key::Char('k') => Command::MoveUp(1),
                Key::Char('h') => Command::MoveLeft(1),
                Key::Char('l') => Command::MoveRight(1),
                Key::Char('x') => Command::Delete(Direction::F),
                Key::Down => Command::MoveDown(1),
                Key::Up => Command::MoveUp(1),
                Key::Left => Command::MoveLeft(1),
                Key::Right => Command::MoveRight(1),
                Key::Char('i') => Command::ChangeMode(Mode::Insert),
                Key::Char('b') => Command::Scroll(1),
                Key::Char('v') => Command::Scroll(-1),
                Key::Char('r') => Command::RecompileSelf,
                Key::Char(' ') => Command::Save,
                _ => Command::Unknown,
            }
        }
    }
}
