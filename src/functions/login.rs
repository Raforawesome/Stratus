#![allow(dead_code)]
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let mut home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Error: Failed to get home directory. Check your operating system.");
        std::process::exit(1);
    });
    home_dir = home_dir.join(".stratus").join("data");
    std::fs::create_dir_all(&home_dir).unwrap_or_else(|_| {
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

pub fn register(name: &str, password: &str) -> Result<(), std::io::Error> {
    let dir: PathBuf = get_data_dir();
    let hashed_pass: String = libpasta::hash_password(password);
    let mut contents: String = String::new();
    contents += name;
    contents += "\n";
    contents += &hashed_pass;
    std::fs::write(&dir.join(".hp"), contents.as_bytes())?;
    Ok(())
}

pub fn try_login(password: &str) -> bool {
    let dir: PathBuf = get_data_dir();
    let contents: String = std::fs::read_to_string(dir.join(".hp")).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read config file. Check read perms in home directory?");
        std::process::exit(1);
    });
    let decoded: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    assert_eq!(decoded.len(), 2, "Error: Credential file corrupted!");
    libpasta::verify_password(decoded[1], password)
}
