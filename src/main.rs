use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("money_database.db")?;

    if let Err(e) = create_tables(&conn) {
        println!("create_tables() error: {}", e);
    }

    Ok(())
}

/// Creates two tables, **receipt** to save all the expenses
/// and **ledger** to know who owes money to whom.
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists receipt (
            id integer primary key,
            payer text not null,
            amount integer not null,
            description text not null
        )",
        [],
    )?;

    conn.execute(
        "create table if not exists ledger (
            id integer primary key,
            borrower text not null,
            amount integer not null,
            owner text not null
        )",
        [],
    )?;

    Ok(())
}
