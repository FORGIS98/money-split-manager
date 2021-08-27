use rusqlite::{params, Connection, Result};

/// Creates two tables, **receipt** to save all the expenses
/// and **ledger** to know who owes money to whom.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS receipt (
            id INTEGER PRIMARY KEY,
            payer TEXT NOT NULL,
            amount INTEGER NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ledger (
            id INTEGER PRIMARY KEY,
            borrower TEXT NOT NULL,
            amount INTEGER NOT NULL,
            owner TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_receipt(conn: &Connection, payer: &str, amount: &u32, description: &str) -> Result<()> {

    conn.execute(
        "INSERT INTO receipt (payer, amount, description) VALUES (?1, ?2, ?3)", 
        params![payer, amount, description]
    )?;

    Ok(())
}
