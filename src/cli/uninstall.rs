use std::fs;
use std::path::Path;

pub fn run(name: &str) {
    let target = Path::new(".jadio_modules").join(name);
    if !target.exists() {
        eprintln!("Error: package '{}' not found", name);
        return;
    }

    fs::remove_dir_all(&target).expect("Failed to remove package");
    println!("Uninstalled package '{}'", name);
}
