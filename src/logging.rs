use std::fs::OpenOptions;
use std::io::Write;

pub fn debug(s: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/tmp/crb.log")
        .expect("could not open log file");
    file.write_all(s.as_bytes())
        .expect("could not write to log file");
    file.write_all("\n".as_bytes())
        .expect("could not write to log file");
}
