use rusqlite::Connection;
use std::{collections::HashMap, error::Error, fs, io, io::ErrorKind, path::Path};

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
        "WELCOME to money-split-manager, this not to friendly terminal experience is going to try to walk you through the app :D");

    println!(""); // Air :D

    println!(
        "So, first things first, do you want to create a new manager? Write \"yes\" or \"no\" please:");

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
            if !Path::new(&manager_path).exists() {
                conn = Connection::open(&manager_path)?;
                break;
            } else {
                println!("...this manager already exist, try another name, like \"look_im_a_dumb_human\": ");
            }
        }
    } else {
        // Show the user a list of available data_bases
        // and let him choose one of them
        println!("So this are all the available managers, choose one of them please, (of course, just give me the number): ");

        let mut all_managers = HashMap::new();

        for files in fs::read_dir("./managers/")? {
            let files = files?;
            all_managers.insert(
                (all_managers.len() + 1) as i32,
                files.path().display().to_string(),
            );
        }

        println!("{:#?}", all_managers);

        loop {
            if all_managers.len() == 0 {
                panic!(
                    "There is no managers here...they are maybe on vacation...as I wish I was..."
                );
            }

            let mut all_user_interaction = String::new();
            io::stdin()
                .read_line(&mut all_user_interaction)
                .expect("Failed to read line...to bad :/");

            println!(""); // Air :D

            let all_user_interaction: i32 = match all_user_interaction.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("...I said a number! (Hate this humans)");
                    continue;
                }
            };

            if all_user_interaction < 1 || all_user_interaction > all_managers.len() as i32 {
                println!("COME ON! You don't even try, I need a number from the list: ");
                continue;
            }

            let manager_path = all_managers.get(&all_user_interaction).unwrap();
            conn = Connection::open(&manager_path)?;

            break;
        }
    }

    if let Err(e) = database::create_tables(&conn) {
        println!("create_tables() error: {}", e);
    }

    if let Err(e) = database::add_receipt(&conn, "Paula", &30, "Pizza") {
        println!("add_receipt() error: {}", e);
    }

    if let Err(e) = database::add_ledger(&conn, "Jorge", &15, "Paula") {
        println!("add_ledger() error: {}", e);
    }

    println!(
        "Now that you have a manager, what do you need from him? (Don't bother him if you don't have to, I like him): "
    );

    println!(""); // Air :D

    println!(
        "1: Print al receipts\n
         2: Add receipt\n
         3: Print ledger\n
         So, choose a number: "
    );

    println!(""); // Air :D

    let mut number: i32;
    loop {
        let mut all_user_interaction = String::new();
        io::stdin()
            .read_line(&mut all_user_interaction)
            .expect("Failed to read line...to bad :/");

        println!(""); // Air :D

        number = match all_user_interaction.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("...I said a number! (They never learn...)");
                continue;
            }
        };

        if number < 1 || number > 3 {
            println!("OMG! Are you stupid? I need a number from the list: ");
            continue;
        }
        break;
    }

    match number {
        1 => database::print_receipt(&conn).unwrap(),
        2 => {},
        3 => database::print_ledger(&conn).unwrap(),
        _ => {
            panic!("This shouldn't explode :/");
        }
    }

    Ok(())
}
