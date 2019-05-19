use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    match tree() {
        Ok(()) => println!("Ok"),
        Err(err) => println!("Err: {}", err),
    }
}

fn tree() -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;
    println!("Current dir: {:?}", current_dir);

    let mut raw_path_string = String::new();
    io::stdin()
        .read_line(&mut raw_path_string)
        .expect("Failed to read line");

    let mut path = PathBuf::new();
    path.push(current_dir);
    path.push(raw_path_string.trim());
    let mut queue = vec![path];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();
        println!("{:?}", path);
        if !path.is_dir() {
            continue;
        }
        let results = fs::read_dir(path)?;
        for entry in results {
            let entry = entry?;
            let entry_path = entry.path();
            queue.push(entry_path);
        }
    }

    Ok(())
}
