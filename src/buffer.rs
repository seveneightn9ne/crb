use std::env;
use std::fs;
use std::io;
use std::io::Read;

struct Buffer {
   contents: String,
}

impl Buffer {
   fn new(contents: String) -> Buffer {
       Buffer {
	contents: contents
       }
   }

   fn load_from_file(path: String) -> Result<Buffer, io::Error> {
       Ok(Buffer{contents: try!(read_file(path))})
   }

   fn print(&self) {
       println!("{}", self.contents)
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
