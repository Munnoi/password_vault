use std::io::{self, Write};
use std::sync::Mutex;

#[derive(Debug)]
#[allow(dead_code)]
struct Credential {
    site: String,
    username: String,
    password: String,
}

static CREDENTIALS: Mutex<Vec<Credential>> = Mutex::new(Vec::new());

fn add_entry() {
    println!("Please enter the following values:");

    let mut values: String = String::new();
    print!("Site name, username & password (enter in a comma(',') separated way): ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut values).expect("Failed to read line!");
    let values_vec: Vec<String> = values.trim().split(',').map(|s| s.trim().to_string()).collect();

    CREDENTIALS.lock().unwrap().push(Credential { site: values_vec[0].clone(), username: values_vec[1].clone(), password: values_vec[2].clone()});
    println!("{:?}", CREDENTIALS);
}

fn main() {
    println!("Welcome to VaultX!");

    loop {
        println!("Choose your option:");
        println!("1. Add password");
        println!("8. Exit");

        let mut input = String::new();

        let _choice: u8;

        print!("Please enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout!");

        io::stdin().read_line(&mut input).expect("Failed to read line!");

        // choice = input.trim().parse().expect("Please type a valid number!");

        match input.trim() {
            "1" => {
                println!("Adding...")
                add_entry();
            }
            "8" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}