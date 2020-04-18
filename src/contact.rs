use chrono::{offset::TimeZone, Datelike, NaiveDate, Utc};

use std::convert::TryInto;
use std::fmt;

pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub address: String,
    pub email: String,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = NaiveDate::parse_from_str(&self.date_of_birth, "%Y-%m-%d").unwrap();
        let age = age(date.year(), date.month(), date.day());
        write!(
            f,
            "===========================================\nName: {}, {}\nDate of birth: {} ({})\nAddress: {}\nEmail: {}\n",
            self.last_name, self.first_name, self.date_of_birth, age, self.address, self.email
        )
    }
}

impl Contact {
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: String,
        address: String,
        email: String,
    ) -> Contact {
        Contact {
            first_name: capitalize(&first_name),
            last_name: capitalize(&last_name),
            date_of_birth,
            address: capitalize(&address),
            email,
        }
    }
}

struct Contacts(pub Vec<Contact>);

impl fmt::Display for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, contact| {
            result.and_then(|_| writeln!(f, "{}", contact))
        })
    }
}

pub fn age(year: i32, month: u32, day: u32) -> u32 {
    let date_of_birth = Utc.ymd(year, month, day);
    let now = Utc::now().date();
    (now.signed_duration_since(date_of_birth).num_days() / 365)
        .try_into()
        .unwrap()
}

fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
