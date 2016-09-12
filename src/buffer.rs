use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use logging;
use mode::Command;
use std::cmp;
use std::iter::FromIterator;

use errors::CrbError;

// A reference to a position.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Anchor {
    id: i64,
}

#[derive(PartialEq, Eq)]
pub struct Wrap {
    style: WrapStyle,
    width: i32,
    // Whether the cursor moves between visual lines or buffer lines.
    vismove: bool,
}

#[derive(PartialEq, Eq)]
pub enum WrapStyle {
    Truncate,
    Hard,
    Word,
}

impl Wrap {
    pub fn default(width: i32) -> Wrap {
        Wrap {
            style: WrapStyle::Truncate,
            width: width,
            vismove: false,
        }
    }
}

// Private structure containing position data.
#[derive(Clone)]
struct Position {
    line: i32,
    // Offset from the beginning of the line.
    // 0 is before the first character.
    // len(line) is after the last character.
    offset: i32,
    // The offset that this will snap back to when moving to a longer line.
    wishful_offset: Option<i32>,
}

struct Line {
    text: String,
}

#[derive(Debug)]
pub struct Display {
    pub x: i32,
    pub y: i32,
    pub symbol: Symbol,
}

#[derive(Debug)]
pub enum Symbol {
    Char(char),
    Anchor(Anchor),
    Void,
}

pub struct Buffer {
    contents: Vec<Line>,
    pub file_path: Option<String>,
    pub unsaved: bool,
    pub newfile: bool,

    // Map from anchor id to position.
    anchors: HashMap<i64, Position>,
    next_anchor_id: i64,
}

impl Buffer {
    /** Creators **/

    pub fn load_from_file(path: &str) -> Result<Buffer, io::Error> {
        let s = try!(read_file(path));
        let contents: Vec<Line> = s.split("\n")
            .map(|x| Line { text: x.to_string() })
            .collect();
        let mut buf = Buffer::empty();
        buf.contents = contents;
        buf.file_path = Some(path.to_string());
        buf.newfile;
        Ok(buf)
    }

    pub fn new_file(path: &str) -> Buffer {
        let mut buf = Buffer::empty();
        buf.file_path = Some(path.to_string());
        buf
    }

    pub fn empty() -> Buffer {
        Buffer {
            contents: Vec::new(),
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
            wishful_offset: None,
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
                let len = self.line(p2.line).unwrap_or("").chars().count() as i32;
                p2.offset = cmp::min(cmp::max(0, p2.offset + n), len);
                p2
            }
            Command::MoveDown(n) => {
                p2.line = cmp::min(cmp::max(0, p2.line + n), self.count_lines());
                let len = self.line(p2.line).unwrap_or("").chars().count() as i32;
                let wish = match p2.wishful_offset {
                    None => p2.offset,
                    Some(x) => x,
                };
                p2.offset = cmp::min(wish, len);
                p2.wishful_offset = match p2.offset < wish {
                    true => Some(wish),
                    false => None,
                };
                p2
            }
            _ => return Err(CrbError::new("unsupported move command")),
        };
        self.anchors.insert(anchor.id, p3);
        Ok(())
    }

    fn new_anchor_id(&mut self) -> i64 {
        self.next_anchor_id += 1;
        self.next_anchor_id - 1
    }

    pub fn insert_text_before(&mut self, anchor: &Anchor, text: char) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let mut pos: Position = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let cur_line = self.contents[pos.line as usize].text.clone();
        let (before, after) = cur_line.split_at(pos.offset as usize);
        self.contents[pos.line as usize] =
            Line { text: before.to_string() + &text.to_string() + &after };
        // TODO move anchors on this line
        pos.offset += 1;
        self.anchors.insert(anchor.id, pos);
        Ok(())
    }

    pub fn delete_at(&mut self, anchor: &Anchor) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let mut pos: Position = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let cur_line = self.contents[pos.line as usize].text.clone();
        let (before, after) = cur_line.split_at(pos.offset as usize);
        let before_argh: String = before.chars().take(before.len() - 1).collect();
        self.contents[pos.line as usize] = Line { text: before_argh.to_string() + after };
        pos.offset -= 1;
        self.anchors.insert(anchor.id, pos);
        Ok(())
    }

    /** Observers **/
    /** Observers **/

    pub fn line(&self, i: i32) -> Option<&str> {
        self.contents.get(i as usize).map(|l| l.text.as_str())
    }

    pub fn count_lines(&self) -> i32 {
        self.contents.len() as i32
    }

    pub fn display(&self, start_line: i32, height: i32, wrap: Wrap) -> Vec<Display> {
        // TODO better overallocation number.
        let mut v = Vec::with_capacity((height * wrap.width + 20) as usize);
        if wrap != Wrap::default(wrap.width) {
            panic!("TODO unsupported wrap");
        }
        // For each line.
        let it = self.contents
            .iter()
            .skip(start_line as usize)
            .take(height as usize)
            .enumerate();
        for (i, line) in it {
            let i = i as i32;
            let mut linetext = line.text.chars();
            let mut charsleft = true;
            let la = self.line_anchors(i);
            let mut ancposes = la.iter();
            let mut next_ancpos = ancposes.next();
            // For each character.
            for j in 0..wrap.width {
                // Send anchor.
                loop {
                    if let Some(&(a_id, p)) = next_ancpos {
                        if p.offset == j {
                            v.push(Display {
                                x: j,
                                y: i,
                                symbol: Symbol::Anchor(Anchor { id: *a_id }),
                                // symbol: Symbol::Void,
                            });
                            next_ancpos = ancposes.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                // TODO anchor off the end.

                // Send character.
                let mut s = Symbol::Void;
                // TODO anchors
                if charsleft {
                    if let Some(c) = linetext.next() {
                        s = Symbol::Char(c);
                    } else {
                        charsleft = false;
                    }
                }
                v.push(Display {
                    x: j,
                    y: i,
                    symbol: s,
                });
            }
        }
        v
    }

    fn line_anchors(&self, i: i32) -> Vec<(&i64, &Position)> {
        self.anchors
            .iter()
            .filter_map(|id_p| match id_p.1.line == i {
                true => Some(id_p),
                false => None,
            })
            .collect()
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
