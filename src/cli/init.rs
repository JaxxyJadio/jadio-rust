use std::fs;
use std::path::Path;

pub fn run() {
    let modules_dir = Path::new(".jadio_modules");
    let config_dir = Path::new("jadio_config");

    if !modules_dir.exists() {
        fs::create_dir_all(modules_dir).expect("Failed to create .jadio_modules");
        println!("Created .jadio_modules/");
    } else {
        println!(".jadio_modules/ already exists");
    }

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create jadio_config");
        println!("Created jadio_config/");
    } else {
        println!("jadio_config/ already exists");
    }
}
