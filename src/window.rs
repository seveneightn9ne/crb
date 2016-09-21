use std::sync::{Arc, Mutex};
use std::cmp;

use buffer::{Buffer, Anchor};
use geometry::{Point, Size};
use mode::{Command, Direction, Mode};
use buffer::{Display, Wrap};
use errors::{CrbResult, CrbError};
use state::State;
use logging;

pub struct Window {
    pub buf: Mutex<Buffer>,
    state: Arc<Mutex<State>>,

    /// Location on the screen.
    pub topleft: Point,
    pub size: Size,
    /// What line the top is at
    pub scroll: i32,

    pub mode: Mode,
    cursors: Vec<Anchor>,
    wrap: Wrap,
    index: i32,
}

fn do_with_state<F, T>(statelock: &Mutex<State>, func: F) -> T
    where F: Fn(&mut State) -> T
{
    let mut state = statelock.lock().unwrap();
    let t = func(&mut *state);
    t
}

impl Window {
    pub fn new(buf: Mutex<Buffer>, topleft: Point, size: Size, state: Arc<Mutex<State>>) -> Window {
        let mut cursors = Vec::new();
        {
            let mut buf = buf.lock().unwrap();
            let a1 = buf.new_anchor();
            cursors.push(a1);
        }
        Window {
            buf: buf,
            index: do_with_state(&*state, |s| {
                let n = s.next_window_index;
                s.next_window_index += 1;
                n
            }),
            state: state,
            topleft: topleft,
            size: size,
            scroll: 0,
            cursors: cursors,
            mode: Mode::Normal,
            wrap: Wrap::default(size.width),
        }
    }

    pub fn save(&mut self) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        buf.save()
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
        let index = "[".to_string() + &self.index.to_string() + &"] ".to_string();
        let rest = match buf.file_path {
            Some(ref thing) => {
                match buf.newfile {
                    true => thing.clone() + &" (new file)".to_string(),
                    false => thing.clone(),
                }
            }
            None => "empty buffer".to_string(),
        };
        unsaved_prefix + &index + &rest
    }

    pub fn move_cursors(&mut self, m: &Command) -> CrbResult<()> {
        // TODO unlocking and then re-locking to call another method
        // probably has the wrong multi-threading guarantees.
        let delta = {
            let mut buf = self.buf.lock().unwrap();
            for anchor in self.cursors.iter() {
                // TODO: this is not good error handling.
                try!(buf.move_anchor(anchor, m));
            }
            let (dataline, wrapline) =
                try!(buf.get_anchor_line(self.cursors.last().unwrap(), &self.wrap));
            let bottom = self.scroll + self.size.height;
            // TODO: handle wrap
            logging::debug(&format!("{} {}", dataline, wrapline).to_owned());
            let up = cmp::min(0, dataline - self.scroll);
            let down = cmp::max(0, dataline - bottom + 2);
            let delta = up + down;
            delta
        };
        try!(self.scroll(&Command::Scroll(delta)));
        Ok(())
    }

    pub fn display<F>(&self, f: F)
        where F: FnMut(&Display)
    {
        let buf = self.buf.lock().unwrap();
        buf.display(self.scroll as usize, self.size, &self.wrap, f);
    }

    pub fn insert(&mut self, c: char) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            try!(buf.insert_text_before(anchor, c));
        }
        Ok(())
    }

    pub fn insert_s(&mut self, s: &str) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for c in s.chars() {
            for anchor in self.cursors.iter() {
                try!(buf.insert_text_before(anchor, c));
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        buf.clear()
    }

    pub fn delete(&mut self, d: Direction) -> CrbResult<()> {
        let mut buf = self.buf.lock().unwrap();
        for anchor in self.cursors.iter() {
            try!(buf.delete_at(anchor, &d));
        }
        Ok(())
    }

    pub fn scroll(&mut self, c: &Command) -> CrbResult<()> {
        let buf = self.buf.lock().unwrap();
        if let Command::Scroll(n) = *c {
            self.scroll += n;
            // TODO this needs to happen whenever the buffer changes size.
            self.scroll = cmp::max(0, cmp::min(self.scroll, buf.count_lines()));
            Ok(())
        } else {
            Err(CrbError::new(&format!("invalid scroll command {:?}", c)))
        }
    }
}
