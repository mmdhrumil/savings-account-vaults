use crate::constants::{STD_DAYS_IN_MONTH, STD_SECONDS_IN_DAY};

pub fn calculate_interest(
    last_interest_timestamp: u64,
    current_timestamp: u64,
    vault_balance: u64,
    interest_per_month: u64
) -> u64 {
    assert!(current_timestamp >= last_interest_timestamp);

    let duration_diff = current_timestamp - last_interest_timestamp;

    let interest_amount = (interest_per_month as u128)
    .checked_mul(vault_balance as u128).unwrap()
    .checked_div(100_u128).unwrap();

    let total_interest = interest_amount
        .checked_mul(duration_diff as u128).unwrap()
        .checked_div(STD_DAYS_IN_MONTH as u128).unwrap()
        .checked_div(STD_SECONDS_IN_DAY as u128).unwrap();

    (total_interest) as u64
}