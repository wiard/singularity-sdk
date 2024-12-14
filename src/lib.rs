pub mod anchor;
pub mod matching;
pub mod utxo;
pub mod verification;

// Example of exposing a function from the UTXO module
pub use utxo::process_utxo;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
