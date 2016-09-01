extern crate rustbox;

mod buffer;

use std::default::Default;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::Key;

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

    rustbox.print(1, 1, rustbox::RB_NORMAL, Color::White, Color::Black, "Oi!");

    rustbox.print(1,
                  3,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "Press 'q' to quit.");

    let mut cursory = 4;
    loop {
        rustbox.set_cursor(10, cursory);

        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('j') => {
                        cursory += 1;
                    }
                    Key::Char('k') => {
                        cursory -= 1;
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
