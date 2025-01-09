use sha2::{Sha256, Digest};

/// Generates a Merkle tree root hash from a list of data
pub fn generate_merkle_root(data: Vec<String>) -> String {
    if data.is_empty() {
        return "".to_string();
    }

    let mut hashes: Vec<String> = data.into_iter().map(|d| {
        let mut hasher = Sha256::new();
        hasher.update(d);
        format!("{:x}", hasher.finalize())
    }).collect();

    while hashes.len() > 1 {
        hashes = hashes.chunks(2).map(|chunk| {
            let mut hasher = Sha256::new();
            hasher.update(chunk[0].as_bytes());
            if chunk.len() > 1 {
                hasher.update(chunk[1].as_bytes());
            }
            format!("{:x}", hasher.finalize())
        }).collect();
    }

    hashes[0].clone()
}

/// Verifies a Merkle proof against the root
pub fn verify_merkle_proof(leaf: &str, proof: Vec<String>, root: &str) -> bool {
    let mut current_hash = {
        let mut hasher = Sha256::new();
        hasher.update(leaf);
        format!("{:x}", hasher.finalize())
    };

    for sibling in proof {
        let mut hasher = Sha256::new();
        if current_hash < sibling {
            hasher.update(current_hash.as_bytes());
            hasher.update(sibling.as_bytes());
        } else {
            hasher.update(sibling.as_bytes());
            hasher.update(current_hash.as_bytes());
        }
        current_hash = format!("{:x}", hasher.finalize());
    }

    current_hash == *root
}

