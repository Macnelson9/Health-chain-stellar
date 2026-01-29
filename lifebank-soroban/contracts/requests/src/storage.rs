use crate::types::{BloodRequest, DataKey};
use soroban_sdk::{Address, Env, Vec};

/// Get the admin address
pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .expect("Admin not initialized")
}

/// Set the admin address
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

/// Get the current request counter
pub fn get_request_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::RequestCounter)
        .unwrap_or(0)
}

/// Increment and return the next request ID
pub fn increment_request_id(env: &Env) -> u64 {
    let current = get_request_counter(env);
    let next_id = current + 1;
    env.storage()
        .instance()
        .set(&DataKey::RequestCounter, &next_id);
    next_id
}

/// Store a blood request
pub fn set_blood_request(env: &Env, request: &BloodRequest) {
    env.storage()
        .persistent()
        .set(&DataKey::BloodRequest(request.id), request);
}

/// Retrieve a blood request by ID
pub fn get_blood_request(env: &Env, request_id: u64) -> Option<BloodRequest> {
    env.storage()
        .persistent()
        .get(&DataKey::BloodRequest(request_id))
}

/// Check if a hospital is authorized
pub fn is_authorized_hospital(env: &Env, hospital: &Address) -> bool {
    let admin = get_admin(env);
    hospital == &admin
}

/// Check if a blood bank is authorized
pub fn is_authorized_blood_bank(env: &Env, bank: &Address) -> bool {
    let admin = get_admin(env);
    bank == &admin
}
