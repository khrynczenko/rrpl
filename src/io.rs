use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read_file(path: &Path) -> String {
    let content = fs::read_to_string(path);
    if let Err(e) = content {
        println!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }

    content.unwrap()
}

pub fn write_file(path: &Path, new_content: &str) {
    let file = File::create(path);

    if let Err(e) = file {
        println!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }

    let mut file = file.unwrap();

    if let Err(e) = file.write_all(new_content.as_bytes()) {
        println!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }
}
