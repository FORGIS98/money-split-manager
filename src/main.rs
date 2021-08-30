use rusqlite::Connection;
use std::{error::Error, fs, io, io::ErrorKind, path::Path};

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    // We first create the dir that will hold all data_bases
    // in case of course that the dir doesn't exist
    fs::create_dir("./managers").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::PermissionDenied {
            panic!("STOP, I don't have write access...to my own directory :/");
        }
    });

    // First conversation
    println!(
        "WELCOME to money-split-manager, this not to friendly terminal experience is going to try to walk you through the app :D \n\n

        So, first things first, do you want to create a new manager? Write \"yes\" or \"no\" please:
        ");

    let new_manager: bool;
    loop {
        let mut all_user_interaction = String::new();
        io::stdin()
            .read_line(&mut all_user_interaction)
            .expect("Failed to read line...to bad :/");

        println!(""); // Air :D

        all_user_interaction.make_ascii_lowercase();
        let all_user_interaction: &str = all_user_interaction.trim();

        if all_user_interaction.eq("yes") {
            new_manager = true;
            break;
        } else if all_user_interaction.eq("no") {
            new_manager = false;
            break;
        } else {
            print!("Was it so hard to anser \"yes\" or \"no\"? My god..., try again :/\n");
        }
    }

    // Second conversation
    let conn: Connection;
    if new_manager {
        // User wants to create a new data_base
        println!("Nice, a new manager, I see you like to keep things organized. Give the manager a name, be careful, I will not care if you misspell him: ");

        loop {
            let mut all_user_interaction = String::new();
            io::stdin()
                .read_line(&mut all_user_interaction)
                .expect("Failed to read line...to bad :/");

            println!(""); // Air :D

            all_user_interaction.make_ascii_lowercase();
            let all_user_interaction: &str = all_user_interaction.trim();
            let manager_path = format!("./managers/{}.db", all_user_interaction);

            // First let's check that the DB doesn't exist already
            if !std::path::Path::new(&manager_path).exists() {
                conn = Connection::open(&manager_path)?;
                break;
            } else {
                println!("...this manager already exist, try another name, like \"look_im_a_dumb_human\": ");
            }
        }
    } else {
        // Show the user a list of available data_bases
        // and let him choose one of them
    }

    let conn = Connection::open("money_database.db")?;

    if let Err(e) = database::create_tables(&conn) {
        println!("create_tables() error: {}", e);
    }

    if let Err(e) = database::add_receipt(&conn, "Paula", &30, "Pizza") {
        println!("add_receipt() error: {}", e);
    }

    if let Err(e) = database::add_ledger(&conn, "Jorge", &15, "Paula") {
        println!("add_ledger() error: {}", e);
    }

    Ok(())
}
