use std::fs;

pub fn run() {
    let path = ".jadio_modules";
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Installed packages:");
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    println!("- {}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(_) => {
            println!("No .jadio_modules folder found. Have you run 'jadio init'?");
        }
    }
}
