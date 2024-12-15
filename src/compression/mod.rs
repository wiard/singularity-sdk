/// Compression module for Outixs
///
/// This module provides functionality to compress and decompress transaction data
/// for efficient storage and processing within Outixs.

pub fn compress_data(data: &str) -> Vec<u8> {
    // Simulate compression (replace with actual algorithm like Gzip or similar)
    data.as_bytes().to_vec()
}

pub fn decompress_data(data: &[u8]) -> String {
    // Simulate decompression (replace with actual algorithm)
    String::from_utf8(data.to_vec()).expect("Failed to decompress data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression() {
        let original = "Test transaction data for compression";
        let compressed = compress_data(original);
        let decompressed = decompress_data(&compressed);
        assert_eq!(original, decompressed);
    }
}

