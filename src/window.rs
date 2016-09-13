use std::sync::Mutex;

use buffer::{Buffer, Anchor};
use geometry::{Point, Size};
use mode::{Command, Mode};
use buffer::{Display, Wrap};
use errors::CrbResult;

pub struct Window {
    pub buf: Mutex<Buffer>,
    pub topleft: Point,
    pub size: Size,
    pub mode: Mode,
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
            cursors: cursors,
            mode: Mode::Normal,
        }
    }

    // pub fn line(&self, i: i32) -> &str {
    //     let mut buf = self.buf.lock().unwrap();
    //     let s = buf.line(i).map_or("", |s| s.to_owned());
    //     s
    // }

    pub fn title(&self) -> String {
        let buf = self.buf.lock().unwrap();
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

    pub fn move_cursors(&mut self, m: &Command) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            try!(buf.move_anchor(anchor, m));
        }
        Ok(())
    }

    pub fn display<F>(&self, f: F)
        where F: FnMut(&Display)
    {
        let buf = self.buf.lock().unwrap();
        // TODO use real wrap
        let wrap = Wrap::default(self.size.width);
        let start_line = 0;
        buf.display(start_line, self.size, wrap, f);
    }

    pub fn insert(&mut self, c: char) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            try!(buf.insert_text_before(anchor, c));
        }
        Ok(())
    }

    pub fn delete(&mut self) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            try!(buf.delete_at(anchor));
        }
        Ok(())
    }
}
