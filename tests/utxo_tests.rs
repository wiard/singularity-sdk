use outixs::utxo::storage::{initialize_db, store_utxos};
use rusqlite::Result;
use serde_json::json;

#[test]
fn test_initialize_db() {
    let conn = initialize_db().expect("Failed to initialize database");
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='utxos'")
        .expect("Failed to prepare statement");
    let table_exists: Result<bool> = stmt.exists([]);
    assert!(table_exists.unwrap(), "UTXOs table was not created");
}

#[test]
fn test_store_utxos() {
    let mut conn = initialize_db().expect("Failed to initialize database");

    // Sample UTXO data
    let utxos = json!([
        {
            "txid": "sample_txid1",
            "vout": 0,
            "value": 100000,
            "scriptPubKey": "sample_script_pubkey1"
        },
        {
            "txid": "sample_txid2",
            "vout": 1,
            "value": 50000,
            "scriptPubKey": "sample_script_pubkey2"
        }
    ]);

    store_utxos(&mut conn, &utxos).expect("Failed to store UTXOs");

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM utxos")
        .expect("Failed to prepare statement");
    let count: i64 = stmt
        .query_row([], |row| row.get(0))
        .expect("Failed to query UTXOs count");
    assert_eq!(count, 2, "UTXOs were not stored correctly");
}

#[test]
fn test_query_utxos() {
    let mut conn = initialize_db().expect("Failed to initialize database");

    // Sample UTXO data
    let utxos = json!([
        {
            "txid": "sample_txid1",
            "vout": 0,
            "value": 100000,
            "scriptPubKey": "sample_script_pubkey1"
        },
        {
            "txid": "sample_txid2",
            "vout": 1,
            "value": 50000,
            "scriptPubKey": "sample_script_pubkey2"
        }
    ]);

    store_utxos(&mut conn, &utxos).expect("Failed to store UTXOs");

    let mut stmt = conn
        .prepare("SELECT txid, vout, value, script_pubkey FROM utxos")
        .expect("Failed to prepare statement");
    let mut rows = stmt.query([]).expect("Failed to query UTXOs");

    let mut results = Vec::new();
    while let Some(row) = rows.next().expect("Failed to fetch row") {
        let txid: String = row.get(0).expect("Failed to get txid");
        let vout: i64 = row.get(1).expect("Failed to get vout");
        let value: i64 = row.get(2).expect("Failed to get value");
        let script_pubkey: String = row.get(3).expect("Failed to get script_pubkey");
        results.push((txid, vout, value, script_pubkey));
    }

    assert_eq!(results.len(), 2, "Unexpected number of UTXOs returned");
    assert_eq!(
        results[0],
        (
            "sample_txid1".to_string(),
            0,
            100000,
            "sample_script_pubkey1".to_string()
        ),
        "First UTXO did not match expected values"
    );
    assert_eq!(
        results[1],
        (
            "sample_txid2".to_string(),
            1,
            50000,
            "sample_script_pubkey2".to_string()
        ),
        "Second UTXO did not match expected values"
    );
}

