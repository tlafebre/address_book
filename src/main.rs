use std::io;
use std::io::*;

mod contact;
mod db;
mod env;
mod fs;

use contact::Contact;
use db::Database;
use fs::mkdir_p;

#[derive(Debug)]
enum Commands {
    Add,
    List,
    Quit,
    NoOp,
}

#[allow(unused_must_use)]
fn add(db: &Database) {
    print!("first name: ");
    std::io::stdout().flush();
    let first_name = get_user_input();
    print!("last name: ");
    std::io::stdout().flush();
    let last_name = get_user_input();
    print!("date of birth (YYYY-MM-DD): ");
    std::io::stdout().flush();
    let age = get_user_input();
    print!("address: ");
    std::io::stdout().flush();
    let address = get_user_input();
    print!("email: ");
    std::io::stdout().flush();
    let email = get_user_input();
    let contact = Contact::new(first_name.clone(), last_name.clone(), age, address, email);
    match db.insert(contact) {
        Ok(()) => println!(
            "{} was added to address book!",
            format!("{} {}", first_name, last_name)
        ),
        Err(e) => println!("failed to add to address book: {}", e),
    }
}

fn list(db: &Database) {
    let contacts = db.list_contacts().unwrap();
    for contact in contacts {
        println!("{}", contact);
    }
}

fn parse_command() -> Result<Commands> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let cmd = match input.trim() {
        "1" => Commands::Add,
        "2" => Commands::List,
        "3" => Commands::Quit,
        _ => Commands::NoOp,
    };
    Ok(cmd)
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().to_string()
}

#[allow(unused_must_use)]
fn menu() {
    println!("Address Book: ");
    println!(" 1) Add new contact");
    println!(" 2) List all contacs");
    println!(" 3) Quit");
    println!();
    print!("> ");
    io::stdout().flush();
}

fn quit() {
    std::process::exit(0);
}

fn check_dirs() {
    let data_dir = env::data_dir();
    if !data_dir.exists() {
        match mkdir_p(data_dir) {
            Ok(()) => (), //println!("data directory created"),  TODO: MOVE TO LOGGING
            Err(e) => println!("failed to create data directory: {}", e),
        }
    }
}

fn check_database() -> Result<Database> {
    let data_dir = env::data_dir();

    let dbs: Vec<_> = data_dir
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().unwrap() == "db")
        .collect();

    if dbs.is_empty() {
        let db = Database {
            path: env::path_to_db(),
        };
        db.create_table();
        Ok(db)
    } else {
        let db = Database {
            path: dbs.get(0).unwrap().clone(),
        };
        Ok(db)
    }
}

fn main() {
    check_dirs();
    let db = check_database().unwrap();
    loop {
        menu();
        match parse_command().unwrap() {
            Commands::Add => add(&db),
            Commands::List => list(&db),
            Commands::Quit => quit(),
            Commands::NoOp => quit(),
        }
    }
}
