use std::{
    env::current_exe,
    fs::{read_to_string, write},
    path::PathBuf,
};

const PRE_COMMIT_PATH: &str = ".git/hooks/pre-commit";

pub fn install_hook() {
    let path = PathBuf::from(PRE_COMMIT_PATH);
    let mut file = read_to_string(&path).unwrap_or(String::from(""));

    let mut current_path = current_exe()
        .unwrap_or(PathBuf::new())
        .as_path()
        .to_string_lossy()
        .to_string();

    if cfg!(windows) {
        current_path = current_path.replace("\\", "/");
        current_path = current_path.replace(":/", "/");
        current_path = "/".to_string() + current_path.as_str();
    }

    file += format!("\n\n{}", current_path).as_str();

    if !file.starts_with("#!/") {
        file = String::from("#!/bin/sh\n\n") + &file;
    }

    write(path, file).expect("Failed to install pre-commit hook!");

    println!("Successfully installed pre-commit hook!");
}
