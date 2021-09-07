use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Receipt {
    id: i32,
    payer: String,
    amount: f32,
    description: String,
}

#[derive(Debug)]
struct Ledger {
    id: i32,
    borrower: String,
    amount: f32,
    owner: String,
}

/// Creates two tables, **receipt** to save all the expenses
/// and **ledger** to know who owes money to whom.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS receipt (
            id INTEGER PRIMARY KEY,
            payer TEXT NOT NULL,
            amount DECIMAL NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ledger (
            id INTEGER PRIMARY KEY,
            borrower TEXT NOT NULL,
            amount DECIMAL NOT NULL,
            owner TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_receipt(conn: &Connection, payer: &str, amount: &f32, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO receipt (payer, amount, description) VALUES (?1, ?2, ?3)",
        params![payer, amount, description],
    )?;

    Ok(())
}

pub fn add_ledger(conn: &Connection, borrower: &str, amount: &f32, owner: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO ledger (borrower, amount, owner) VALUES (?1, ?2, ?3)",
        params![borrower, amount, owner],
    )?;

    Ok(())
}

pub fn print_receipt(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM receipt;")?;
    let rows = stmt.query_map([], |row| {
        Ok(Receipt {
            id: row.get(0)?,
            payer: row.get(1)?,
            amount: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    for row in rows {
        println!("{:?}", row.unwrap());
    }

    Ok(())
}

pub fn print_ledger(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM ledger;")?;
    let rows = stmt.query_map([], |row| {
        Ok(Ledger {
            id: row.get(0)?,
            borrower: row.get(1)?,
            amount: row.get(2)?,
            owner: row.get(3)?,
        })
    })?;

    for row in rows {
        println!("{:?}", row.unwrap());
    }

    Ok(())
}
