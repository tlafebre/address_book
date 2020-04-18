use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn data_dir() -> PathBuf {
    let mut data_dir = home_dir();
    data_dir.push("data");

    data_dir
}

pub fn path_to_db() -> PathBuf {
    let mut path = data_dir();
    path.push("address_book.db");

    path
}
