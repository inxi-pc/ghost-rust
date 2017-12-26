use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::env;

fn get_root_dir() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn get_file_content<T: AsRef<Path>>(path: &T) -> Result<String, io::Error> {
    let mut file_content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn get_file_path<T: AsRef<Path>>(path: &T) -> PathBuf {
    let mut root_dir: PathBuf = get_root_dir();
    root_dir.push(path);

    root_dir
}

#[test]
fn load_file_content_test() {
    use dao::common;

    let default_json_file_path: PathBuf = get_file_path::<&str>(&common::DEFAULT_SETTINGS_FILE_DIR);
    match get_file_content::<PathBuf>(&default_json_file_path) {
        Ok(content) => println!("{}", content),
        Err(ref err) => println!("{}", err),
    }
}
