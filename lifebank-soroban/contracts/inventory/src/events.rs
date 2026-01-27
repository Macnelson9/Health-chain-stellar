use crate::types::{BloodRegisteredEvent, BloodType};
use soroban_sdk::{Address, Env, Symbol};

/// Emit a BloodRegistered event
///
/// # Arguments
/// * `env` - Contract environment
/// * `blood_unit_id` - Unique ID of the registered blood unit
/// * `bank_id` - Blood bank that registered the unit
/// * `blood_type` - Type of blood
/// * `quantity_ml` - Quantity in milliliters
/// * `expiration_timestamp` - When the unit expires
pub fn emit_blood_registered(
    env: &Env,
    blood_unit_id: u64,
    bank_id: &Address,
    blood_type: BloodType,
    quantity_ml: u32,
    expiration_timestamp: u64,
) {
    let registered_at = env.ledger().timestamp();

    let event = BloodRegisteredEvent {
        blood_unit_id,
        bank_id: bank_id.clone(),
        blood_type,
        quantity_ml,
        expiration_timestamp,
        registered_at,
    };

    env.events()
        .publish((Symbol::new(env, "blood_registered"),), event);
}
