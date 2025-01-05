use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::{Write, BufRead};
use std::fs::File;

lazy_static! {
    static ref BITMAP_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// Inscribe relay data to Bitmap and persist to a file
#[allow(dead_code)]
pub fn inscribe_to_bitmap(parcel: &str, relay_data: &str) -> Result<(), &'static str> {
    let mut store = BITMAP_STORE.lock().unwrap();
    if store.contains_key(parcel) {
        return Err("Parcel already inscribed");
    }
    store.insert(parcel.to_string(), relay_data.to_string());

    // Persist to file
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("bitmap_inscriptions.txt")
        .unwrap();
    writeln!(file, "{}: {}", parcel, relay_data).unwrap();

    println!("Inscribed to Bitmap Parcel: {}, Data: {}", parcel, relay_data);
    Ok(())
}

// Load inscriptions from file on startup
#[allow(dead_code)]
pub fn load_inscriptions_from_file() {
    let file = File::open("bitmap_inscriptions.txt");
    if let Ok(file) = file {
        let reader = std::io::BufReader::new(file);
        let mut store = BITMAP_STORE.lock().unwrap();
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    store.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
        }
    }
}

// Fetch inscription from memory
#[allow(dead_code)]
pub fn get_inscription(parcel: &str) -> Option<String> {
    let store = BITMAP_STORE.lock().unwrap();
    store.get(parcel).cloned()
}

// Clear all inscriptions
#[allow(dead_code)]
pub fn clear_inscriptions() {
    let mut store = BITMAP_STORE.lock().unwrap();
    store.clear();
    std::fs::remove_file("bitmap_inscriptions.txt").ok();
    println!("All inscriptions cleared.");
}

// List all inscriptions
#[allow(dead_code)]
pub fn list_all_inscriptions() {
    let store = BITMAP_STORE.lock().unwrap();
    for (parcel, data) in store.iter() {
        println!("Parcel: {}, Data: {}", parcel, data);
    }
}

