use rusqlite::{params, Connection, Result};

pub fn initialize_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS utxos (
            txid TEXT PRIMARY KEY,
            vout INTEGER,
            value INTEGER,
            script_pubkey TEXT
        )",
        [],
    )?;
    Ok(conn)
}

pub fn store_utxos(conn: &Connection, utxos: Vec<(String, i32, i64, String)>) -> Result<()> {
    for (txid, vout, value, script_pubkey) in utxos {
        conn.execute(
            "INSERT INTO utxos (txid, vout, value, script_pubkey) VALUES (?1, ?2, ?3, ?4)",
            params![txid, vout, value, script_pubkey],
        )?;
    }
    Ok(())
}

