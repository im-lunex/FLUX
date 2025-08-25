use crate::ui::display::*;
use std::io::{self, Write};

pub fn main_menu() {
    loop {
        println!("\nMain Menu:");
        println!("[1] Create new user");
        println!("[2] Login");
        println!("[3] Exit");
        print!("-> Enter your choice [1-3]: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_user_creation(),
            "2" => handle_login(),
            "3" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}
