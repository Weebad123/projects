use crate::contact::Contact;


pub struct ContactManager {
    pub contacts: Vec<Contact>,
    pub next_id: usize,
}


impl ContactManager {

    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_contact(&mut self, name: String, email: String, phone: String) {
        let id = self.next_id;
        let contact = Contact {
            id,
            name,
            email,
            phone
        };

        self.contacts.push(contact);

        self.next_id += 1;
    }

    pub fn view_contacts(&self) {
        if self.contacts.is_empty() {
            println!("No contacts found!")
        } else {
            for i in self.contacts.iter() {
                println!("[{}]: {} | {} | {}", i.id, i.name, i.email, i.phone);
            }
        }
    }

    pub fn search_contact(&self, query: &str) -> Vec<&Contact> {

        // search could be by email or phone or name
        let my_query = query.trim().to_lowercase();

        self.contacts.iter()
            .filter(|c| 
                c.email.contains(&my_query) || 
                c.name.contains(&my_query) || 
                c.phone.contains(&my_query)
            ).collect::<Vec<&Contact>>()
    }

    pub fn delete_contact(&mut self, id: usize) -> bool {
        // deletion based on id
        let original_length = self.contacts.len();
        self.contacts.retain(|c| c.id != id);

        // length check to ensure deletion
        original_length != self.contacts.len()
    }
}