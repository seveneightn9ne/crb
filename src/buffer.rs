use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use logging;

use errors::CrbError;

// Reference to a position.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Anchor {
    id: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum AnchorKind {
    Cursor,
}

enum Offset {
    FromStart(usize),
    End,
}

struct Position {
    line: usize,
    word: usize,
    offset: Offset,
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
            word: 0,
            offset: Offset::FromStart(0),
        };
        self.anchors.insert(a.id, p);
        a
    }

    pub fn move_anchor(&mut self, anchor: Anchor, offset: i32) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let p: &mut Position = try!(self.anchors.get_mut(&anchor.id).ok_or(err));
        // TODO
        p.offset = Offset::FromStart(4);
        p.line += 1;
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

    pub fn anchor_at(&self, anchor: Anchor, rel_x: i32, rel_y: i32) -> bool {
        self.anchors.get(&anchor.id).map_or(false, |p| (p.line as i32) == rel_y)
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(fs::File::open(path));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}
