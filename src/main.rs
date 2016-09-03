extern crate rustbox;

mod buffer;
mod geometry;
mod window;
mod graphics;

use std::default::Default;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::error::Error;

use rustbox::{Color, RustBox};
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

    let buf1 = match env::args().nth(1) {
        Some(path) => try!(buffer::Buffer::load_from_file(path)),
        None => buffer::Buffer::empty(),
    };

    let width = rustbox.width() as i32;
    let height = rustbox.height() as i32;

    let window1 = Window::new(&buf1, Point::new(0, 0), Size::new(width, height));

    let mut cursor_y = 0;
    loop {
        rustbox.set_cursor(0, cursor_y);
        graphics::render(&rustbox, &window1);

        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('j') => {
                        cursor_y += 1;
                    }
                    Key::Char('k') => {
                        cursor_y -= 1;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    println!("Hello, world!");

    if env::args().count() > 2 {
        println!("Usage: crb [<file>]");
        std::process::exit(1);
    }

    match env::args().nth(1) {
        Some(path) => {
            println!("opening file: {}", path);
            open_file(path);
        }
        None => println!("no file specified. making my job easy ;)"),
    }

    Ok(())
}

fn open_file(path: String) {
    match read_file(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Could not open file: {}", e),
    }
}

fn read_file(path: String) -> Result<String, io::Error> {
    let mut f = try!(fs::File::open(path));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}

// fn prompt(p: String) -> Result<String, std::io::Error> {
//     print!("{}", p);
//     try!(io::stdout().flush());
//     let mut s = String::new();
//     try!(io::stdin().read_line(&mut s));
//     Ok(s)
// }
