use buffer::Buffer;
use geometry::{Point, Size};

pub struct Window<'a> {
    pub buf: &'a Buffer,
    pub topleft: Point,
    pub size: Size,
}

impl<'a> Window<'a> {
    pub fn new(buf: &Buffer, topleft: Point, size: Size) -> Window {
        Window {
            buf: buf,
            topleft: topleft,
            size: size,
        }
    }

    pub fn line(&self, i: i32) -> &str {
        self.buf.line(i)
    }

    pub fn title(&self) -> String {
        let unsaved_prefix = match self.buf.unsaved {
            true => "*".to_string(),
            false => " ".to_string(),
        };
        let rest = match self.buf.file_path {
            Some(ref thing) => {
                match self.buf.newfile {
                    true => thing.clone() + &" (new file)".to_string(),
                    false => thing.clone(),
                }
            }
            None => "empty buffer".to_string(),
        };
        unsaved_prefix + &rest
    }
}
