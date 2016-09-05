use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use logging;
use mode::Command;
use std::cmp;

use errors::CrbError;

// Reference to a position.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Anchor {
    id: usize,
}

pub struct Wrap {
    style: WrapStyle,
    // Whether the cursor moves between visual lines or buffer lines.
    vismove: bool,
}

pub enum WrapStyle {
    Truncate,
    // Width in characters
    Hard(i32),
    Word(i32),
}

#[derive(Clone)]
struct Position {
    line: i32,
    // Offset from the beginning of the line.
    offset: i32,
}

pub struct Buffer {
    pub contents: String,
    pub file_path: Option<String>,
    pub unsaved: bool,
    pub newfile: bool,

    // Map from anchor id to position.
    anchors: HashMap<usize, Position>,
    next_anchor_id: usize,
}

impl Buffer {
    /** Creators **/

    pub fn load_from_file(path: &str) -> Result<Buffer, io::Error> {
        Ok(Buffer {
            contents: try!(read_file(&path)),
            file_path: Some(path.to_string()),
            unsaved: false,
            newfile: false,
            anchors: HashMap::new(),
            next_anchor_id: 0,
        })
    }

    pub fn new_file(path: &str) -> Buffer {
        Buffer {
            contents: "".to_string(),
            file_path: Some(path.to_string()),
            unsaved: false,
            newfile: true,
            anchors: HashMap::new(),
            next_anchor_id: 0,
        }
    }

    pub fn empty() -> Buffer {
        Buffer {
            contents: "".to_string(),
            file_path: None,
            unsaved: false,
            newfile: true,
            anchors: HashMap::new(),
            next_anchor_id: 0,
        }
    }

    /** Mutators **/

    pub fn new_anchor(&mut self) -> Anchor {
        let a = Anchor { id: self.new_anchor_id() };
        let p = Position {
            line: 0,
            offset: 0,
        };
        self.anchors.insert(a.id, p);
        a
    }

    pub fn move_anchor(&mut self, anchor: Anchor, m: &Command) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let mut p2: Position = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let m = canonicalize_move(m);
        let p3: Position = match m {
            Command::MoveRight(n) => {
                let len = self.line(p2.line).chars().count() as i32;
                p2.offset = cmp::min(cmp::max(0, p2.offset + n), len);
                p2
            }
            Command::MoveDown(n) => {
                p2.line = cmp::min(cmp::max(0, p2.line + n), self.count_lines());
                let len = self.line(p2.line).chars().count() as i32;
                p2.offset = cmp::min(p2.offset, len);
                p2
            }
            _ => return Err(CrbError::new("unsupported move command")),
        };
        self.anchors.insert(anchor.id, p3);
        Ok(())
    }

    fn new_anchor_id(&mut self) -> usize {
        self.next_anchor_id += 1;
        self.next_anchor_id - 1
    }

    /** Observers **/

    pub fn line(&self, i: i32) -> &str {
        match self.contents.split('\n').nth(i as usize) {
            Some(s) => s,
            None => "",
        }
    }

    pub fn count_lines(&self) -> i32 {
        self.contents.split('\n').count() as i32
    }

    pub fn anchor_at(&self, anchor: Anchor, x: i32, y: i32) -> bool {
        self.anchors.get(&anchor.id).map_or(false, |p| {
            let mx = (p.line as i32) == y;
            let my = (p.offset as i32) == x;
            mx && my
        })
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(fs::File::open(path));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}

fn canonicalize_move(mov: &Command) -> Command {
    match mov.clone() {
        Command::MoveLeft(n) => Command::MoveRight(-n),
        Command::MoveUp(n) => Command::MoveDown(-n),
        m @ _ => m,
    }
}
