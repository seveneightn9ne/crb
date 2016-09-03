use std::fs;
use std::io;
use std::io::Read;

pub struct Buffer {
    pub contents: String,
    pub file_path: Option<String>,
}

impl Buffer {

    /** Creators **/

    pub fn load_from_file(path: String) -> Result<Buffer, io::Error> {
        Ok(Buffer {
	    contents: try!(read_file(&path)),
	    file_path: Some(path)
	})
    }

    pub fn empty() -> Buffer {
    	Buffer{contents: "".to_string(), file_path: None}
    }

    /** Observers **/

    pub fn head(&self, n: usize) -> String {
        self.contents[..n].to_string()
    }

    pub fn line(&self, i: i32) -> &str {
        match self.contents.split('\n').nth(i as usize) {
            Some(s) => s,
            None => "",
        }
    }
}

fn read_file(path: &String) -> Result<String, io::Error> {
    let mut f = try!(fs::File::open(path));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}