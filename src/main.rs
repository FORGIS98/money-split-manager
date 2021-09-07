use rusqlite::Connection;
use std::{collections::HashMap, error::Error, fs, io, io::ErrorKind, io::Write, path::Path};

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
        "\n\nWELCOME to money-split-manager, this not to friendly terminal experience is going to try to walk you through the app :D"
    );

    println!(""); // Air :D

    print!(
        "So, first things first, do you want to create a new manager? Write \"yes\" or \"no\" please: "
    );
    io::stdout().flush().unwrap();

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
            print!("Was it so hard to answer \"yes\" or \"no\"? My god..., try again: ");
            io::stdout().flush().unwrap();
        }
    }

    // Second conversation
    let conn: Connection;
    if new_manager {
        // User wants to create a new data_base
        print!("Nice, a new manager, I see you like to keep things organized. Give the manager a name, be careful, I will not care if you misspell him: ");
        io::stdout().flush().unwrap();

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
        println!("So, this are all the available managers, choose one of them please, (of course, just give me the number): ");

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

    println!("Now that you have a manager, what do you need from him? (Don't bother him if you don't have to, I don't want to loose my time): ");

    println!(""); // Air :D

    print!("1: Print al receipts\n2: Add receipt\n3: Print ledger\n\nSo, choose a number: ");
    io::stdout().flush().unwrap();

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
                print!("...I said a number! (They never learn...): ");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        if number < 1 || number > 3 {
            print!("OMG! Are you stupid? I need a number from the list: ");
            io::stdout().flush().unwrap();
            continue;
        }
        break;
    }

    match number {
        1 => database::print_receipt(&conn).unwrap(),
        2 => {
            println!(
                "Okay, let's add a receipt, I need you to write the next information separated with a semicolon \";\". \nFirst I need to know who has paid the bill, then I need the amount and finally a description. I need the 3 things. Here is an example: \n\nMr.Rust;34.94;Fruit, drinks and yogurts. \n\nI hope it is clear :)\n"
            );
            add_new_receipt(&conn)
        }
        3 => database::print_ledger(&conn).unwrap(),
        _ => {
            panic!("This shouldn't explode :/");
        }
    }

    Ok(())
}

fn add_new_receipt(conn: &Connection) {
    print!("Enter a receipt: ");
    io::stdout().flush().unwrap();
    loop {
        let mut all_user_interaction = String::new();
        io::stdin()
            .read_line(&mut all_user_interaction)
            .expect("Failed to read line...to bad :/");
        let mut all_user_interaction = all_user_interaction.trim().split(";");

        let payer: &str = all_user_interaction.next().unwrap();
        let amount: f32 = all_user_interaction.next().unwrap().parse().unwrap();
        let description: &str = all_user_interaction.next().unwrap();

        if let Err(e) = database::add_receipt(conn, payer, &amount, description) {
            println!("add_receipt() error: {}", e);
        }

        print!("Do you want to add a new receipt? Write \"yes\" or \"no\" please: ");
        io::stdout().flush().unwrap();

        let mut all_user_interaction = String::new();
        io::stdin()
            .read_line(&mut all_user_interaction)
            .expect("Failed to read line...to bad :/");

        println!(""); // Air :D

        all_user_interaction.make_ascii_lowercase();
        let all_user_interaction: &str = all_user_interaction.trim();

        if all_user_interaction.eq("yes") {
            continue;
        } else if all_user_interaction.eq("no") {
            break;
        } else {
            print!("Are you using your brain? Just write \"yes\" or \"no\" please: ");
            io::stdout().flush().unwrap();
        }
    }
}
