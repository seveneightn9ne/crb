use std::fs;
use std::io;
use std::io::{Read, Write};
use std::collections::HashMap;
use mode::{Command, Direction};
use std::cmp;
use std::cmp::Ordering;
use std;
use settings::Settings;
use settings;
use std::sync::{Arc, Mutex};

use rustbox::Color;
use geometry;
use errors::{CrbError, CrbResult};

/// A reference to a position.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Anchor {
    id: i64,
}

#[derive(PartialEq, Eq)]
pub struct Wrap {
    style: WrapStyle,
    width: i32,
    /// Whether the cursor moves between visual lines or buffer lines.
    vismove: bool,
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
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

/// Private structure containing position data.
#[derive(Clone, PartialOrd)]
struct Position {
    line: i32,
    /// Offset from the beginning of the line.
    /// 0 is before the first character.
    /// len(line) is after the last character.
    offset: i32,
    /// The offset that this will snap back to when moving to a longer line.
    wishful_offset: Option<i32>,
}
impl Position {
    fn new(line: i32, offset: i32) -> Position {
        Position {
            line: line,
            offset: offset,
            wishful_offset: None,
        }
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.line, self.offset) == (other.line, other.offset)
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.line, self.offset).cmp(&(other.line, other.offset))
    }
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
    ColorChar(char, Color),
    Anchor(Anchor),
    Void,
}

pub struct Buffer {
    // Invariant: contents.len() > 0
    contents: Vec<Line>,
    pub file_path: Option<String>,
    pub unsaved: bool,
    pub newfile: bool,
    settings: Arc<Mutex<Settings>>,

    // Map from anchor id to position.
    anchors: HashMap<i64, Position>,
    next_anchor_id: i64,
}

impl Buffer {
    /** Creators **/

    pub fn load_from_file(path: &str, settings: Arc<Mutex<Settings>>) -> Result<Buffer, io::Error> {
        let s = try!(read_file(path));
        let mut contents: Vec<Line> = s.split("\n")
            .map(|x| Line { text: x.to_string() })
            .collect();
        if contents.len() == 0 {
            contents.push(Line { text: "".to_string() });
        }
        let mut buf = Buffer::empty(settings);
        buf.contents = contents;
        buf.file_path = Some(path.to_string());
        buf.newfile = false;
        Ok(buf)
    }

    pub fn new_file(path: &str, settings: Arc<Mutex<Settings>>) -> Buffer {
        let mut buf = Buffer::empty(settings);
        buf.file_path = Some(path.to_string());
        buf
    }

    pub fn empty(settings: Arc<Mutex<Settings>>) -> Buffer {
        Buffer {
            contents: vec![Line { text: "".to_string() }],
            file_path: None,
            unsaved: false,
            newfile: true,
            anchors: HashMap::new(),
            next_anchor_id: 0,
            settings: settings,
        }
    }

    /** Mutators **/

    pub fn save(&mut self) -> CrbResult<()> {
        // TODO keep track of whether changed
        if let Some(ref file_path) = self.file_path {
            let f = try!(fs::File::create(file_path)
                .map_err(|e| CrbError::new(&format!("error while opening to save {}", e))));
            let mut w = io::BufWriter::new(f);
            for line in &self.contents {
                try!(w.write_all(line.text.as_bytes())
                    .map_err(|e| CrbError::new(&format!("error while saving {}", e))));
                try!(w.write_all(b"\n")
                    .map_err(|e| CrbError::new(&format!("error while saving {}", e))));
            }
            Ok(())
        } else {
            return Err(CrbError::new("cannot save with no file path"));
        }
    }

    pub fn new_anchor(&mut self) -> Anchor {
        let a = Anchor { id: self.new_anchor_id() };
        let p = Position::new(0, 0);
        self.anchors.insert(a.id, p);
        a
    }

    pub fn move_anchor(&mut self, anchor: &Anchor, m: &Command) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let pos = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let pos2 = try!(self.move_pos(&pos, m));
        self.anchors.insert(anchor.id, pos2);
        Ok(())
    }

    fn move_pos(&mut self, pos: &Position, m: &Command) -> Result<Position, CrbError> {
        let mut p2 = pos.clone();
        let m = canonicalize_move(m);
        let p3: Position = match m {
            Command::MoveRight(n) => {
                let len = self.line(p2.line).unwrap_or("").chars().count() as i32;
                p2.offset = cmp::min(cmp::max(0, p2.offset + n), len);
                p2.wishful_offset = None;
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
        Ok(p3)
    }

    fn new_anchor_id(&mut self) -> i64 {
        self.next_anchor_id += 1;
        self.next_anchor_id - 1
    }

    pub fn insert_text_before(&mut self, anchor: &Anchor, text: char) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let pos: Position = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let err = CrbError::new("delete_at_pos: no line at pos");
        let cur_line = try!(self.contents.get(pos.line as usize).ok_or(err)).text.clone();
        let (before, after) = cur_line.split_at(pos.offset as usize);
        if text == '\n' {
            self.contents[pos.line as usize] = Line { text: before.to_string() };
            self.contents.insert((pos.line + 1) as usize, Line { text: after.to_string() });
            self.anchors.insert(anchor.id, Position::new(pos.line + 1, 0));
            Ok(())
        } else {
            self.contents[pos.line as usize] =
                Line { text: before.to_string() + &text.to_string() + &after };
            // TODO move anchors on this line
            self.move_anchor(anchor, &Command::MoveRight(1))
        }
    }

    pub fn delete_at(&mut self, anchor: &Anchor, d: &Direction) -> Result<(), CrbError> {
        let err = CrbError::new("no such anchor");
        let pos = try!(self.anchors.get(&anchor.id).ok_or(err)).clone();
        let pos: Position = match *d {
            Direction::B => pos.clone(),
            Direction::F => try!(self.move_pos(&pos, &Command::MoveRight(1))),
        };
        self.delete_at_pos(&pos)
    }

    /// Delete backwards from the position.
    fn delete_at_pos(&mut self, pos: &Position) -> Result<(), CrbError> {
        let err = CrbError::new("delete_at_pos: no line at pos");
        let cur_line = try!(self.contents.get(pos.line as usize).ok_or(err)).text.clone();
        if pos.offset == 0 && pos.line == 0 {
            Ok(())
        } else if pos.offset == 0 {
            let old_text = self.contents[(pos.line - 1) as usize].text.clone();
            self.contents[(pos.line - 1) as usize] = Line { text: old_text.clone() + &cur_line };

            // move anchors the line that got merged up
            for (_, p) in self.anchors.iter_mut() {
                if p.line == pos.line {
                    let p2 = Position::new(p.line - 1,
                                           p.offset + (old_text.chars().count() as i32));
                    *p = p2;
                }
            }

            self.contents.remove(pos.line as usize);
            Ok(())
        } else {
            let (before, after) = cur_line.split_at(pos.offset as usize);
            let before_argh: String = before.chars().take(before.len() - 1).collect();
            self.contents[pos.line as usize] = Line { text: before_argh.to_string() + after };

            // move anchors on this line
            for (_, p) in self.anchors.iter_mut() {
                if p.line == pos.line && p.offset >= pos.offset {
                    let p2 = Position::new(p.line, cmp::max(0, p.offset - 1));
                    *p = p2
                }
            }

            Ok(())
        }
    }

    pub fn clear(&mut self) -> CrbResult<()> {
        self.contents.clear();
        self.contents.push(Line { text: "".to_string() });
        self.anchors = self.anchors
            .iter()
            .map(|(&a, _)| (a, Position::new(0, 0)))
            .collect::<HashMap<_, _>>();
        Ok(())
    }

    /** Observers **/

    pub fn line(&self, i: i32) -> Option<&str> {
        self.contents.get(i as usize).map(|l| l.text.as_str())
    }

    pub fn count_lines(&self) -> i32 {
        self.contents.len() as i32
    }

    /// Calls the closure in scan order on the rectangular area.
    pub fn display<F>(&self, start_line: usize, size: geometry::Size, wrap: &Wrap, mut f: F)
        where F: FnMut(&Display)
    {
        if *wrap != Wrap::default(wrap.width) {
            panic!("TODO unsupported wrap");
        }

        // TODO horizontal scrolling
        // TODO multi-width characters

        let mut buf_y = start_line as usize;
        let mut buf_x = 0 as usize;
        let mut lines = self.contents.iter().skip(buf_y);
        let mut line_chars = to_chars(lines.next()).peekable();
        let anchors_all = self.all_anchors();
        let mut anchors_iter = anchors_all.iter().peekable();

        for view_y in 0..size.height {
            let mut view_x = 0;
            // Draw line numbers
            if buf_y <= self.contents.len() {
                let col_size = self.contents.len().to_string().chars().count();
                let line_num_str = (buf_y + 1).to_string();
                let mut line_num_chars = line_num_str.chars();
                let offset = col_size - line_num_chars.clone().count();
                let color = match self.settings.lock().unwrap().get("color-linenumbers") {
                    Some(&settings::Value::Color(c)) => c,
                    _ => Color::White,
                };
                for i in 0..col_size {
                    if offset <= i {
                        let d = Display {
                            x: view_x,
                            y: view_y,
                            symbol: Symbol::ColorChar(line_num_chars.next().unwrap(), color),
                        };
                        f(&d);
                    }
                    view_x += 1;
                }
            }


            for view_x in (view_x + 1)..size.width {
                let mut did_anchor = false;
                if let Some(tpl) = anchors_iter.peek() {
                    let tpl: &(&i64, &Position) = tpl;
                    let anchor_id: &i64 = tpl.0;
                    let pos: &Position = tpl.1;
                    if (pos.line as usize == buf_y) && (pos.offset as usize == buf_x) {
                        let a = Anchor { id: anchor_id + 0 };
                        let d = Display {
                            x: view_x,
                            y: view_y,
                            symbol: Symbol::Anchor(a),
                        };
                        f(&d);
                        did_anchor = true;
                    }
                }
                if did_anchor {
                    anchors_iter.next();
                }

                let s = match line_chars.next() {
                    Some(c) => Symbol::Char(c),
                    None => Symbol::Void,
                };
                let d = Display {
                    x: view_x,
                    y: view_y,
                    symbol: s,
                };
                f(&d);
                buf_x += 1;
            }
            if line_chars.peek().is_none() {
                // Next buf line
                buf_y += 1;
                buf_x = 0;
                line_chars = to_chars(lines.next()).peekable();
            }
        }
    }

    fn all_anchors(&self) -> Vec<(&i64, &Position)> {
        let mut ans: Vec<(&i64, &Position)> = self.anchors.iter().collect();
        ans.sort_by_key(|x| x.1);
        ans
    }

    /// Get the line of an anchor.
    /// The [0] value is the data line.
    /// The [1] value is the wrap line offset from that line.
    pub fn get_anchor_line(&self, a: &Anchor, _: &Wrap) -> CrbResult<(i32, i32)> {
        let err = CrbError::new("no such anchor");
        let p = try!(self.anchors.get(&a.id).ok_or(err));
        // TODO handle wrap
        Ok((p.line, 0))
    }
}


fn to_chars(oli: Option<&Line>) -> std::str::Chars {
    match oli {
        Some(y) => y.text.chars(),
        None => "".chars(),
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
