use std::fmt;

pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub age: String,
    pub address: String,
    pub email: String,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "===========================================\nName: {}, {}\nAge: {}\nAddress: {}\nEmail: {}\n",
            self.last_name, self.first_name, self.age, self.address, self.email
        )
    }
}

impl Contact {
    pub fn new(
        first_name: String,
        last_name: String,
        age: String,
        address: String,
        email: String,
    ) -> Contact {
        Contact {
            first_name: capitalize(&first_name),
            last_name: capitalize(&last_name),
            age,
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

fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
