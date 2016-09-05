extern crate rustbox;

mod buffer;
mod geometry;
mod window;
mod graphics;
mod errors;

use std::default::Default;
use std::env;
use std::error::Error;
use std::sync::Mutex;

use rustbox::RustBox;
use rustbox::Key;

use window::Window;
use geometry::{Point, Size};

fn main() {
    match startup() {
        Ok(_) => {}
        Err(e) => println!("Fatal error: {}", e),
    }
}

fn startup() -> Result<(), Box<Error>> {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => return Err(Box::new(e)),
    };

    let mut buf1 = match env::args().nth(1) {
        Some(path) => {
            match buffer::Buffer::load_from_file(&path) {
                Ok(buffer) => buffer,
                _ => buffer::Buffer::new_file(&path),
            }
        }
        None => buffer::Buffer::empty(),
    };

    let buf1 = Mutex::new(buf1);

    let width = rustbox.width() as i32;
    let height = rustbox.height() as i32;

    let mut window1 = Window::new(buf1, Point::new(0, 0), Size::new(width, height));

    loop {
        graphics::render(&rustbox, &window1);

        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('j') => window1.move_cursor_vert(1),
                    Key::Char('k') => window1.move_cursor_vert(-1),
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    Ok(())
}
