use crate::error::ContractError;
use crate::types::BloodType;
use soroban_sdk::Env;

/// Maximum request quantity (5000ml)
pub const MAX_REQUEST_QUANTITY_ML: u32 = 5000;
/// Minimum request quantity (50ml)
pub const MIN_REQUEST_QUANTITY_ML: u32 = 50;
/// Maximum days in future for required_by timestamp
pub const MAX_DAYS_IN_FUTURE: u64 = 30;
pub const SECONDS_PER_DAY: u64 = 86400;

/// Validate blood request parameters
///
/// Checks:
/// - Quantity is within acceptable range (50-5000ml)
/// - Required_by is in the future
/// - Required_by is not too far in the future (max 30 days)
pub fn validate_request_creation(
    env: &Env,
    quantity_ml: u32,
    required_by: u64,
) -> Result<(), ContractError> {
    // Validate quantity
    if quantity_ml < MIN_REQUEST_QUANTITY_ML || quantity_ml > MAX_REQUEST_QUANTITY_ML {
        return Err(ContractError::InvalidQuantity);
    }

    let current_time = env.ledger().timestamp();

    // Required_by must be in the future
    if required_by <= current_time {
        return Err(ContractError::InvalidTimestamp);
    }

    // Required_by shouldn't be too far in the future
    let max_future = current_time + (MAX_DAYS_IN_FUTURE * SECONDS_PER_DAY);
    if required_by > max_future {
        return Err(ContractError::InvalidTimestamp);
    }

    Ok(())
}

/// Validate delivery address is not empty
pub fn validate_delivery_address(address: &soroban_sdk::String) -> Result<(), ContractError> {
    if address.len() == 0 {
        return Err(ContractError::InvalidInput);
    }
    Ok(())
}

/// Validate blood type is valid
pub fn validate_blood_type(_blood_type: &BloodType) -> Result<(), ContractError> {
    // All BloodType variants are valid by construction
    Ok(())
}

/// Check if request has exceeded its deadline
pub fn is_request_overdue(required_by: u64, current_time: u64) -> bool {
    current_time > required_by
}

/// Calculate time remaining until deadline in seconds
pub fn time_until_deadline(required_by: u64, current_time: u64) -> i64 {
    required_by as i64 - current_time as i64
}
