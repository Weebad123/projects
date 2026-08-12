pub mod contact {

    #[derive(Debug)]
    pub struct Contact {
        pub id: usize,
        pub name: String,
        pub email: String,
        pub phone: String,
    }
}

pub use contact::Contact;