#![allow(dead_code)]
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let mut home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Error: Failed to get home directory. Check your operating system.");
        std::process::exit(1);
    });
    home_dir = home_dir.join(".stratus").join("data");
    let _ = std::fs::create_dir_all(&home_dir).unwrap_or_else(|_| {
        eprintln!(
            "Error in creating directory. Double check write permissions in your home directory?"
        );
        std::process::exit(1);
    });
    home_dir
}

pub fn is_registered() -> bool {
    get_data_dir().join(".hp").exists()
}

pub fn try_login(password: &str) -> bool {
    true
}
