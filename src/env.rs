use std::path::PathBuf;

pub fn current_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn data_dir() -> PathBuf {
    let mut data_dir = current_dir();
    data_dir.push("data");

    data_dir
}

pub fn path_to_db() -> PathBuf {
    let mut path = data_dir();
    path.push("address_book.db");

    path
}