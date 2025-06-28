use std::fs;
use std::path::{Path, PathBuf};

pub fn run(source: &str) {
    let src = Path::new(source);
    if !src.is_dir() {
        eprintln!("Error: source '{}' is not a directory", source);
        return;
    }

    let dest = PathBuf::from(".jadio_modules").join(
        src.file_name().expect("Failed to get package name"),
    );

    if dest.exists() {
        eprintln!("Error: package already installed at {:?}", dest);
        return;
    }

    fs::create_dir_all(".jadio_modules").expect("Failed to ensure .jadio_modules exists");

    fs_extra::dir::copy(
        src,
        ".jadio_modules",
        &fs_extra::dir::CopyOptions::new().copy_inside(true),
    ).expect("Failed to copy package");

    println!("Installed package to {:?}", dest);
}
