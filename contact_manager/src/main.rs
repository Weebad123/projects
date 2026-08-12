mod contact;
mod contact_manager;
mod ui;

use contact_manager::ContactManager;
use ui::{get_input, get_input_number, show_menu};

fn main() {
    println!("Contact Manager");

    let mut contact_manager = ContactManager::new();

    loop {
        show_menu();

        let user_choice = get_input("Select an Option: (1 - 5)");

        match user_choice.as_str() {
            "1" => handle_add_contact(&mut contact_manager),
            "2" => handle_view_contacts(&contact_manager),
            "3" => handle_search_contacts(&contact_manager),
            "4" => handle_delete_contact(&mut contact_manager),
            "5" => {
                println!("Exiting Contact Manager!! Bye!!");
                break;
            },
            _ => {
                println!("Invalid option. Select Option (1 - 5)");
            }
        }
    }
}

fn handle_add_contact(contact: &mut ContactManager) {
    let name = get_input("Enter Your Name: ");
    if name.is_empty() { println!("Name cannot be empty") }

    let email = get_input("Enter your email: ");
    let phone = get_input("Enter your Phone number: ");

    println!("Adding contact.......");
    contact.add_contact(name, email, phone);
    println!("Contact added successfully");

}


fn handle_view_contacts(contact: &ContactManager) {
    println!("Showing All Contacts!!");
    contact.view_contacts();
}

fn handle_search_contacts(contact: &ContactManager) {
    let search_query = get_input("Search by email/name/phone. Enter here: ");
    if search_query.is_empty() {
        println!("Please enter a search term");
        return;
    }

    let search_results = contact.search_contact(&search_query);

    // search results could be empty or not
    if search_results.is_empty() {
        println!("No results found for query {}!", &search_query);
    } else {
        println!("Search results: {} found", search_results.len());
        for (i, each_result) in search_results.iter().enumerate() {
            println!(
                "\nContact {} => [{}]: {} | {} | {} ",
                i, each_result.id, each_result.name, each_result.email, each_result.phone
            );
        }
    }
}


fn handle_delete_contact(contact: &mut ContactManager) {
    if contact.contacts.is_empty() {
        return;
    }

    let id_to_delete = match get_input_number("Enter an ID number to delete") {
        Some(0) => {
            println!("Deletion cancelled");
            return;
        },
        Some(id) => id,
        None => {
            println!("Invalid ID");
            return;
        }
    };

    if contact.delete_contact(id_to_delete) {
        println!("Contact deleted successfully!");

    } else {
        println!("Specified contact Not Found!");
    }

}
