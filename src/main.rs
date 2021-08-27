use rusqlite::{Connection, Result};

mod database;

fn main() -> Result<()> {
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
