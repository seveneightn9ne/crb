use rustbox::Key;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Mode {
    Normal,
    Insert,
}

pub enum Command {
    MoveLeft(u8),
    MoveRight(u8),
    MoveUp(u8),
    MoveDown(u8),
    Quit,
    Insert(char),
    Unknown,
    ChangeMode(Mode),
}

type ModeMap = HashMap<Key, Command>;

pub fn map(mode: Mode, key: Key) -> Command {
    match mode {
        Mode::Insert => {
            match key {
                Key::Char(c) => Command::Insert(c),
                Key::Esc => Command::ChangeMode(Mode::Normal),
                _ => Command::Unknown,
            }
        }
        Mode::Normal => {
            match key {
                Key::Char('q') => Command::Quit,
                Key::Char('j') => Command::MoveUp(1),
                Key::Char('k') => Command::MoveDown(1),
                Key::Char('i') => Command::ChangeMode(Mode::Insert),
                _ => Command::Unknown,
            }
        }
    }
}
