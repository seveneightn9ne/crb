extern crate rustbox;

mod buffer;
mod geometry;
mod window;
mod graphics;
mod errors;
mod logging;
mod mode;

use std::default::Default;
use std::env;
use std::error::Error;
use std::sync::Mutex;

use rustbox::RustBox;
use rustbox::Key;

use window::Window;
use geometry::{Point, Size};

fn main() {
    logging::debug("Startup");
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
                let cmd = mode::map(window1.mode.clone(), key);
                match cmd {
                    mode::Command::Quit => break,
                    mode::Command::MoveUp(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveDown(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveLeft(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveRight(_) => window1.move_cursors(&cmd),
                    mode::Command::Insert(c) => window1.insert(c),
                    mode::Command::ChangeMode(m) => window1.mode = m,
                    _ => {}//TODO show this somewhere
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    Ok(())
}
