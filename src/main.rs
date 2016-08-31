use std::env;
use std::fs;
use std::io;
use std::io::Read;

fn main() {
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
        None => {
            println!("no file specified. making my job easy ;)")
        }
    }
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
