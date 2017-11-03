use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::env;

pub fn load_file_content<T: AsRef<Path>>(path: &T) -> Result<String, io::Error> {
    let mut file_content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn get_root_dir() -> PathBuf {
    env::current_dir().unwrap()
}

#[test]
fn load_file_content_test() {
    let mut root_dir: PathBuf = get_root_dir();
    root_dir.push("src/dao/data/default_settings.json");
    println!("{}", root_dir.display());
    match load_file_content::<PathBuf>(&root_dir) {
        Ok(content) => println!("{}", content),
        Err(ref err) => println!("{}", err),
    }
}