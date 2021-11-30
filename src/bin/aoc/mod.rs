use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

pub fn load(file_name: &str) -> String {
    let path = Path::new(&file_name);

    let mut file = match File::open(&path) {
        Ok(value) => value,
        Err(reason) => panic!("File::open failed: {}", reason),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(reason) => panic!("Read::read_to_string failed: {}", reason),
    };
}

pub fn lines(file_name: &str) -> Vec<String> {
    let mut lines = vec![];
    for line in load(file_name).lines() {
        lines.push(String::from(line));
    }

    return lines;
}
