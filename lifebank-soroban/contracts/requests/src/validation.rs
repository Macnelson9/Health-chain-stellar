use crate::error::ContractError;
use crate::storage::{MAX_REQUEST_WINDOW_DAYS, MIN_REQUEST_WINDOW_SECONDS, SECONDS_PER_DAY};
use soroban_sdk::{Env, String};

/// Minimum quantity for a blood request (100ml = partial unit)
pub const MIN_REQUEST_QUANTITY_ML: u32 = 100;

/// Maximum quantity for a blood request (10000ml = ~22 units)
/// Large enough for major surgeries or emergency situations
pub const MAX_REQUEST_QUANTITY_ML: u32 = 10000;

/// Validate blood request creation parameters
///
/// Checks:
/// - Quantity is within acceptable range (100-10000ml)
/// - required_by is in the future
/// - required_by is not too far in the future (max 30 days)
/// - Delivery address is not empty
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
    delivery_address: &String,
) -> Result<(), ContractError> {
    // Validate quantity
    validate_quantity(quantity_ml)?;

    // Validate required_by timestamp
    validate_required_by(env, required_by)?;

    // Validate delivery address
    validate_delivery_address(delivery_address)?;

    Ok(())
}

/// Validate quantity is within acceptable range
///
/// # Arguments
/// * `quantity_ml` - Quantity in milliliters
///
/// # Returns
/// Ok(()) if valid, Err(InvalidQuantity) if out of range
pub fn validate_quantity(quantity_ml: u32) -> Result<(), ContractError> {
    if quantity_ml < MIN_REQUEST_QUANTITY_ML || quantity_ml > MAX_REQUEST_QUANTITY_ML {
        return Err(ContractError::InvalidQuantity);
    }
    Ok(())
}

/// Validate required_by timestamp
///
/// Checks:
/// - Must be in the future (at least MIN_REQUEST_WINDOW_SECONDS from now)
/// - Must not be too far in the future (max MAX_REQUEST_WINDOW_DAYS)
///
/// # Arguments
/// * `env` - Contract environment
/// * `required_by` - Unix timestamp when blood is required
///
/// # Returns
/// Ok(()) if valid, Err(InvalidRequiredBy) if invalid
pub fn validate_required_by(env: &Env, required_by: u64) -> Result<(), ContractError> {
    let current_time = env.ledger().timestamp();

    // required_by must be in the future (at least 1 hour for processing)
    let min_required_by = current_time + MIN_REQUEST_WINDOW_SECONDS;
    if required_by < min_required_by {
        return Err(ContractError::InvalidRequiredBy);
    }

    // required_by shouldn't be too far in the future (max 30 days)
    let max_required_by = current_time + (MAX_REQUEST_WINDOW_DAYS * SECONDS_PER_DAY);
    if required_by > max_required_by {
        return Err(ContractError::InvalidRequiredBy);
    }

    Ok(())
}

/// Validate that a required_by timestamp hasn't passed
///
/// Used when checking if a request has expired
///
/// # Arguments
/// * `env` - Contract environment
/// * `required_by` - Unix timestamp when blood is required
///
/// # Returns
/// Ok(()) if not expired, Err(RequestExpired) if expired
pub fn validate_not_expired(env: &Env, required_by: u64) -> Result<(), ContractError> {
    let current_time = env.ledger().timestamp();

    if current_time >= required_by {
        return Err(ContractError::RequestExpired);
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
///
/// # Arguments
/// * `delivery_address` - The delivery address string
///
/// # Returns
/// Ok(()) if valid, Err(InvalidDeliveryAddress) if empty
pub fn validate_delivery_address(delivery_address: &String) -> Result<(), ContractError> {
    if delivery_address.len() == 0 {
        return Err(ContractError::InvalidDeliveryAddress);
pub fn validate_delivery_address(address: &soroban_sdk::String) -> Result<(), ContractError> {
    if address.len() == 0 {
        return Err(ContractError::InvalidInput);
    }
    Ok(())
}

/// Validate that required_by allows minimum time for critical requests
///
/// Critical requests need at least 1 hour for processing
/// Urgent requests need at least 4 hours
/// Normal requests need at least 24 hours
///
/// # Arguments
/// * `env` - Contract environment
/// * `required_by` - Unix timestamp when blood is required
/// * `is_critical` - Whether this is a critical urgency request
///
/// # Returns
/// Ok(()) if valid time window, Err if too short
pub fn validate_urgency_time_window(
    env: &Env,
    required_by: u64,
    urgency_weight: u32,
) -> Result<(), ContractError> {
    let current_time = env.ledger().timestamp();
    let time_available = required_by.saturating_sub(current_time);

    // Minimum time windows based on urgency
    // Critical (weight 3): 1 hour minimum
    // Urgent (weight 2): 4 hours minimum
    // Normal (weight 1): 24 hours minimum
    let min_time = match urgency_weight {
        3 => 3600,       // 1 hour for critical
        2 => 4 * 3600,   // 4 hours for urgent
        _ => 24 * 3600,  // 24 hours for normal
    };

    if time_available < min_time {
        return Err(ContractError::InvalidRequiredBy);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Ledger as _, Env};

    fn setup_env_with_time(timestamp: u64) -> Env {
        let env = Env::default();
        env.ledger().set_timestamp(timestamp);
        env
    }

    #[test]
    fn test_validate_quantity_valid() {
        assert!(validate_quantity(100).is_ok()); // Minimum
        assert!(validate_quantity(450).is_ok()); // Standard unit
        assert!(validate_quantity(900).is_ok()); // 2 units
        assert!(validate_quantity(10000).is_ok()); // Maximum
    }

    #[test]
    fn test_validate_quantity_too_low() {
        assert_eq!(validate_quantity(0), Err(ContractError::InvalidQuantity));
        assert_eq!(validate_quantity(50), Err(ContractError::InvalidQuantity));
        assert_eq!(validate_quantity(99), Err(ContractError::InvalidQuantity));
    }

    #[test]
    fn test_validate_quantity_too_high() {
        assert_eq!(
            validate_quantity(10001),
            Err(ContractError::InvalidQuantity)
        );
        assert_eq!(
            validate_quantity(50000),
            Err(ContractError::InvalidQuantity)
        );
    }

    #[test]
    fn test_validate_required_by_valid() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // 2 hours in future (valid)
        let required_by = current_time + 2 * 3600;
        assert!(validate_required_by(&env, required_by).is_ok());

        // 7 days in future (valid)
        let required_by = current_time + 7 * SECONDS_PER_DAY;
        assert!(validate_required_by(&env, required_by).is_ok());

        // 30 days in future (maximum valid)
        let required_by = current_time + 30 * SECONDS_PER_DAY;
        assert!(validate_required_by(&env, required_by).is_ok());
    }

    #[test]
    fn test_validate_required_by_too_soon() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // In the past
        assert_eq!(
            validate_required_by(&env, current_time - 100),
            Err(ContractError::InvalidRequiredBy)
        );

        // At current time
        assert_eq!(
            validate_required_by(&env, current_time),
            Err(ContractError::InvalidRequiredBy)
        );

        // 30 minutes in future (less than minimum 1 hour)
        assert_eq!(
            validate_required_by(&env, current_time + 1800),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_required_by_too_far() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // 31 days in future (exceeds 30 day max)
        let required_by = current_time + 31 * SECONDS_PER_DAY;
        assert_eq!(
            validate_required_by(&env, required_by),
            Err(ContractError::InvalidRequiredBy)
        );

        // 60 days in future
        let required_by = current_time + 60 * SECONDS_PER_DAY;
        assert_eq!(
            validate_required_by(&env, required_by),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_not_expired_valid() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // 1 second in future (not expired)
        assert!(validate_not_expired(&env, current_time + 1).is_ok());

        // 1 hour in future
        assert!(validate_not_expired(&env, current_time + 3600).is_ok());
    }

    #[test]
    fn test_validate_not_expired_expired() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // At current time (expired)
        assert_eq!(
            validate_not_expired(&env, current_time),
            Err(ContractError::RequestExpired)
        );

        // In the past (expired)
        assert_eq!(
            validate_not_expired(&env, current_time - 100),
            Err(ContractError::RequestExpired)
        );
    }

    #[test]
    fn test_validate_delivery_address_valid() {
        let env = Env::default();
        let address = String::from_str(&env, "123 Hospital Street");
        assert!(validate_delivery_address(&address).is_ok());

        let address = String::from_str(&env, "A"); // Minimum 1 character
        assert!(validate_delivery_address(&address).is_ok());
    }

    #[test]
    fn test_validate_delivery_address_empty() {
        let env = Env::default();
        let address = String::from_str(&env, "");
        assert_eq!(
            validate_delivery_address(&address),
            Err(ContractError::InvalidDeliveryAddress)
        );
    }

    #[test]
    fn test_validate_urgency_time_window_critical() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // Critical: needs at least 1 hour
        // Valid: 2 hours
        assert!(validate_urgency_time_window(&env, current_time + 2 * 3600, 3).is_ok());

        // Invalid: 30 minutes
        assert_eq!(
            validate_urgency_time_window(&env, current_time + 1800, 3),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_urgency_time_window_urgent() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // Urgent: needs at least 4 hours
        // Valid: 5 hours
        assert!(validate_urgency_time_window(&env, current_time + 5 * 3600, 2).is_ok());

        // Invalid: 2 hours
        assert_eq!(
            validate_urgency_time_window(&env, current_time + 2 * 3600, 2),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_urgency_time_window_normal() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        // Normal: needs at least 24 hours
        // Valid: 48 hours
        assert!(validate_urgency_time_window(&env, current_time + 48 * 3600, 1).is_ok());

        // Invalid: 12 hours
        assert_eq!(
            validate_urgency_time_window(&env, current_time + 12 * 3600, 1),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_request_creation_all_valid() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        let quantity_ml = 450u32;
        let required_by = current_time + 7 * SECONDS_PER_DAY;
        let delivery_address = String::from_str(&env, "123 Hospital Street");

        assert!(validate_request_creation(&env, quantity_ml, required_by, &delivery_address).is_ok());
    }

    #[test]
    fn test_validate_request_creation_invalid_quantity() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        let required_by = current_time + 7 * SECONDS_PER_DAY;
        let delivery_address = String::from_str(&env, "123 Hospital Street");

        assert_eq!(
            validate_request_creation(&env, 50, required_by, &delivery_address),
            Err(ContractError::InvalidQuantity)
        );
    }

    #[test]
    fn test_validate_request_creation_invalid_required_by() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        let quantity_ml = 450u32;
        let delivery_address = String::from_str(&env, "123 Hospital Street");

        // Too soon
        assert_eq!(
            validate_request_creation(&env, quantity_ml, current_time + 100, &delivery_address),
            Err(ContractError::InvalidRequiredBy)
        );
    }

    #[test]
    fn test_validate_request_creation_invalid_address() {
        let current_time = 1000000u64;
        let env = setup_env_with_time(current_time);

        let quantity_ml = 450u32;
        let required_by = current_time + 7 * SECONDS_PER_DAY;
        let delivery_address = String::from_str(&env, "");

        assert_eq!(
            validate_request_creation(&env, quantity_ml, required_by, &delivery_address),
            Err(ContractError::InvalidDeliveryAddress)
        );
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
