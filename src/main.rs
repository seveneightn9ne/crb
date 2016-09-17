extern crate rustbox;

mod buffer;
mod geometry;
mod window;
mod graphics;
mod errors;
mod logging;
mod mode;
mod hacks;

use std::default::Default;
use std::env;
use std::error::Error;
use std::sync::Mutex;
use std::str;
use std::process;

use rustbox::RustBox;

use window::Window;
use geometry::{Point, Size};
use errors::CrbResult;

fn main() {
    logging::debug("started");

    // Restart loop.
    loop {
        match startup() {
            Ok(true) => {
                let res = hacks::restart();
                if let Err(e) = res {
                    println!("Fatal error restarting: {}", e);
                }
                break;
            }
            Ok(false) => break,
            Err(e) => println!("Fatal error: {}", e),
        }
    }
}

fn startup() -> Result<bool, Box<Error>> {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => return Err(Box::new(e)),
    };

    let buf1 = match env::args().nth(1) {
        Some(path) => {
            match buffer::Buffer::load_from_file(&path) {
                Ok(buffer) => buffer,
                _ => buffer::Buffer::new_file(&path),
            }
        }
        None => buffer::Buffer::empty(),
    };

    let buf1 = Mutex::new(buf1);

    let buf2 = buffer::Buffer::empty();
    let buf2 = Mutex::new(buf2);

    let buf3 = buffer::Buffer::empty();
    let buf3 = Mutex::new(buf3);

    let width = rustbox.width() as i32;
    let height = rustbox.height() as i32;

    let mut window1 = Window::new(buf1, Point::new(0, 0), Size::new(width, height - 10));
    let mut window2 = Window::new(buf2, Point::new(0, height - 10), Size::new(width, 10));
    let mut window3 = Window::new(buf3, Point::new(width / 2, 2), Size::new(width / 2 - 1, 2));

    loop {
        graphics::render(&rustbox, &window1);
        graphics::render(&rustbox, &window2);
        graphics::render(&rustbox, &window3);

        rustbox.present();

        let event = rustbox.poll_event(false);
        match event {
            Ok(rustbox::Event::KeyEvent(key)) => {
                let cmd = mode::map(window1.mode.clone(), key);
                let res = match cmd {
                    mode::Command::Quit => break,
                    mode::Command::MoveUp(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveDown(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveLeft(_) => window1.move_cursors(&cmd),
                    mode::Command::MoveRight(_) => window1.move_cursors(&cmd),
                    mode::Command::Insert(c) => window1.insert(c),
                    mode::Command::Delete => window1.delete(),
                    mode::Command::NewLine => window1.insert('\n'),
                    mode::Command::ChangeMode(m) => {
                        window1.mode = m;
                        Ok(())
                    }
                    mode::Command::RecompileSelf => {
                        // TODO handle error
                        let _ = window2.clear();
                        let res = hacks::recompile();
                        let restart =
                            res.and_then(|output| fill_compilation_buffer(&mut window2, output));
                        if let Ok(true) = restart {
                            return Ok(true);
                        }
                        restart.and(Ok(()))
                    }
                    mode::Command::Save => window1.save(),
                    _ => Ok(()), //TODO show this somewhere
                };
                if let Err(e) = res {
                    logging::debug(&format!("cmd error: {}", e));
                }
            }
            // TODO don't panic...
            Err(e) => panic!("{}", e),
            _ => {}
        }
        // TODO handle errors
        if let Ok(rustbox::Event::KeyEvent(key)) = event {
            let cmd = mode::map(window1.mode.clone(), key);
            let _ = window3.clear();
            let _ = window3.insert_s(&format!("{:?}", event));
            let _ = window3.insert('\n');
            let _ = window3.insert_s(&format!("{:?}", cmd));
        }
    }

    Ok(false)
}

fn fill_compilation_buffer(w: &mut Window, output: process::Output) -> CrbResult<bool> {
    try!(w.clear());
    if output.status.success() {
        try!(w.insert_s("Compilation successful\n\n"));
        match str::from_utf8(&output.stderr) {
            Ok(s) => try!(w.insert_s(s)),
            Err(e) => {
                try!(w.insert_s("<stderr-utf8-error>\n"));
                try!(w.insert_s(&format!("{}", e)));
            }
        };
        try!(w.insert('\n'));
        match str::from_utf8(&output.stdout) {
            Ok(s) => try!(w.insert_s(s)),
            Err(e) => {
                try!(w.insert_s("<stdout-utf8-error>\n"));
                try!(w.insert_s(&format!("{}", e)));
            }
        };
        Ok(true)
    } else {
        try!(w.insert_s("Compilation failed\n\n"));
        Ok(false)
    }
}
