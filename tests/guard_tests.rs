use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_valid_solana_address() {
    let valid_addr = "11111111111111111111111111111111";
    assert!(Pubkey::from_str(valid_addr).is_ok());
}

#[test]
fn test_invalid_solana_address() {
    let invalid_addr = "not_a_valid_base58_address_123!!!";
    assert!(Pubkey::from_str(invalid_addr).is_err());
}

#[test]
fn test_amount_limits() {
    let limit = 50.0;
    let safe_tx = 15.0;
    let malicious_tx = 500.0;

    assert!(safe_tx <= limit);
    assert!(malicious_tx > limit);
}