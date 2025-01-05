use rusqlite::{params, Connection, Result};
use serde_json::Value;

/// Initialize the SQLite database connection
#[allow(dead_code)]
pub fn initialize_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS utxos (
            txid TEXT,
            vout INTEGER,
            value INTEGER,
            script_pubkey TEXT
        )",
        [],
    )?;
    Ok(conn)
}

/// Store UTXOs in the database
#[allow(dead_code)]
pub fn store_utxos(conn: &mut Connection, utxos: &Value) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS utxos (
            txid TEXT,
            vout INTEGER,
            value INTEGER,
            script_pubkey TEXT
        )",
        [],
    )?;

    if let Some(utxo_array) = utxos.as_array() {
        for utxo in utxo_array {
            tx.execute(
                "INSERT INTO utxos (txid, vout, value, script_pubkey) VALUES (?1, ?2, ?3, ?4)",
                params![
                    utxo["txid"].as_str().unwrap_or(""),
                    utxo["vout"].as_i64().unwrap_or(0),
                    utxo["value"].as_i64().unwrap_or(0),
                    utxo["scriptPubKey"].as_str().unwrap_or(""),
                ],
            )?;
        }
    }

    tx.commit()
}

