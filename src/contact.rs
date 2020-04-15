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
            "===========================================\nName: {} {}\nAge: {}\nAddress: {}\nEmail: {}\n",
            self.first_name, self.last_name, self.age, self.address, self.email
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
            first_name,
            last_name,
            age,
            address,
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
