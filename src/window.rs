use std::sync::Mutex;
use buffer::{Buffer, Anchor};
use geometry::{Point, Size};

pub struct Window {
    pub buf: Mutex<Buffer>,
    pub topleft: Point,
    pub size: Size,
    cursors: Vec<Anchor>,
}

impl Window {
    pub fn new(buf: Mutex<Buffer>, topleft: Point, size: Size) -> Window {
        let mut cursors = Vec::new();
        {
            let mut buf = buf.lock().unwrap();
            let a1 = buf.new_anchor();
            cursors.push(a1);
        }
        Window {
            buf: buf,
            topleft: topleft,
            size: size,
            cursors: Vec::new(),
        }
    }

    pub fn line(&self, i: i32) -> String {
        let mut buf = self.buf.lock().unwrap();
        buf.line(i).to_string()
    }

    pub fn title(&self) -> String {
        let mut buf = self.buf.lock().unwrap();
        let unsaved_prefix = match buf.unsaved {
            true => "*".to_string(),
            false => " ".to_string(),
        };
        let rest = match buf.file_path {
            Some(ref thing) => {
                match buf.newfile {
                    true => thing.clone() + &" (new file)".to_string(),
                    false => thing.clone(),
                }
            }
            None => "empty buffer".to_string(),
        };
        unsaved_prefix + &rest
    }

    pub fn char_at(&self, rel_x: i32, rel_y: i32) -> Option<char> {
        let mut buf = self.buf.lock().unwrap();
        // Subtract one for the top bar.
        buf.line(rel_y - 1).chars().skip((rel_x) as usize).next()
    }

    pub fn cursor_at(&self, rel_x: i32, rel_y: i32) -> bool {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            if buf.anchor_at(*anchor, rel_x, rel_y - 1) {
                return true;
            }
        }
        false
    }

    pub fn move_cursor_vert(&mut self, dy: i32) {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            buf.move_anchor(*anchor, 0);
        }
    }
}
