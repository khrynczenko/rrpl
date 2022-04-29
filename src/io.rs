use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

pub fn read_file<T: AsRef<Path>>(path: T) -> String {
    log::info!("Started reading from {:#?}", path.as_ref().as_os_str());

    let content = fs::read_to_string(path.as_ref());
    if let Err(e) = content {
        eprintln!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }

    log::info!("Finished reading from {:#?}", path.as_ref().as_os_str());

    content.unwrap()
}

pub fn write_file<T: AsRef<Path>>(path: T, new_content: &str) {
    let file = File::create(path.as_ref());

    log::info!("Started writing into {:#?}", path.as_ref().as_os_str());

    if let Err(e) = file {
        log::error!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }

    let mut file = file.unwrap();

    if let Err(e) = file.write_all(new_content.as_bytes()) {
        log::error!("{}", e);
        match e.raw_os_error() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        }
    }

    log::info!("Finished writing into {:#?}", path.as_ref().as_os_str());
}

pub fn peform_backup<T: AsRef<Path>>(path: T, content: &str) {
    log::info!("Started backup for {:#?}", path.as_ref().as_os_str());

    let tilde_symbol = OsString::from_str("~");
    if tilde_symbol.is_err() {
        log::error!("Cannot encode `~` symbol using this system' encoding.");
        std::process::exit(1);
    }

    let backup: OsString =
        OsString::from_iter([path.as_ref().as_os_str(), tilde_symbol.unwrap().as_os_str()]);

    write_file(&backup, content);

    log::info!("Finished backup for {:#?}", path.as_ref().as_os_str());
}
