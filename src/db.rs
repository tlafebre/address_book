extern crate rusqlite;

use rusqlite::{params, Connection, Error, Result};
use std::path::PathBuf;

use super::contact;

use contact::Contact;

#[derive(Debug)]
pub struct Database {
    pub path: PathBuf,
}

impl Database {
    pub fn create_table(&self) -> Result<()> {
        let conn = self.connect()?;
        conn.execute(
            "CREATE TABLE contacts (
                    id              INTEGER PRIMARY KEY,
                    first_name      TEXT NOT NULL,
                    last_name       TEXT NOT NULL,
                    date_of_birth   TEXT NOT NULL,
                    address         TEXT NOT NULL,
                    email           TEXT NOT NULL
                    )",
            params![],
        )
        .expect("failed to create table");
        Ok(())
    }
    pub fn connect(&self) -> Result<Connection, Error> {
        let conn = Connection::open(self.path.clone()).expect("failed to connect to database");
        Ok(conn)
    }
    pub fn insert(&self, contact: Contact) -> Result<()> {
        let conn = self.connect()?;
        conn.execute(
            "INSERT INTO contacts (first_name, last_name, date_of_birth, address, email) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![contact.first_name, contact.last_name, contact.date_of_birth, contact.address, contact.email],
        )?;
        Ok(())
    }
    pub fn list_contacts(&self) -> Result<Vec<Contact>, Error> {
        let conn = self.connect().unwrap();
        let mut results = conn.prepare(
            "SELECT first_name, last_name, date_of_birth, address, email FROM contacts 
                          ORDER BY last_name ASC, first_name ASC",
        )?;
        let rows = results.query_map(params![], |row| {
            Ok(Contact {
                first_name: row.get(0)?,
                last_name: row.get(1)?,
                date_of_birth: row.get(2)?,
                address: row.get(3)?,
                email: row.get(4)?,
            })
        })?;
        let mut contacts = Vec::new();
        for contact in rows {
            contacts.push(contact.unwrap());
        }

        Ok(contacts)
    }
}
