use crate::types::{BloodRequest, BloodType, DataKey, RequestStatus, UrgencyLevel};
use soroban_sdk::{Address, Env, Vec};

/// Maximum time window for request fulfillment (30 days)
pub const MAX_REQUEST_WINDOW_DAYS: u64 = 30;

/// Seconds per day constant
pub const SECONDS_PER_DAY: u64 = 86400;

/// Minimum time window for request fulfillment (1 hour for critical)
pub const MIN_REQUEST_WINDOW_SECONDS: u64 = 3600;

// ========== Admin Management ==========

/// Get the admin address
///
/// # Panics
/// Panics if admin has not been set (contract not initialized)
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

/// Check if contract has been initialized (admin is set)
pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

// ========== Hospital Authorization ==========

/// Check if a hospital is authorized to create requests
pub fn is_authorized_hospital(env: &Env, hospital: &Address) -> bool {
    // Admin is always authorized
    if let Some(admin) = env.storage().instance().get::<DataKey, Address>(&DataKey::Admin) {
        if hospital == &admin {
            return true;
        }
    }

    // Check if hospital is in authorized set
    env.storage()
        .persistent()
        .has(&DataKey::AuthorizedHospital(hospital.clone()))
}

/// Authorize a hospital to create blood requests
pub fn authorize_hospital(env: &Env, hospital: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::AuthorizedHospital(hospital.clone()), &true);
}

/// Revoke hospital authorization
pub fn revoke_hospital(env: &Env, hospital: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::AuthorizedHospital(hospital.clone()));
}

// ========== Request ID Generation ==========

/// Get the current request counter value
/// Get the current request counter
pub fn get_request_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::RequestCounter)
        .unwrap_or(0)
}

/// Increment and return the next request ID
///
/// IDs start at 1 and increment sequentially
pub fn increment_request_id(env: &Env) -> u64 {
    let current = get_request_counter(env);
    let next_id = current + 1;
    env.storage()
        .instance()
        .set(&DataKey::RequestCounter, &next_id);
    next_id
}

// ========== Blood Request Storage ==========

/// Store a blood request
pub fn set_blood_request(env: &Env, request: &BloodRequest) {
    env.storage()
        .persistent()
        .set(&DataKey::BloodRequest(request.id), request);
}

/// Get a blood request by ID
pub fn get_blood_request(env: &Env, id: u64) -> Option<BloodRequest> {
    env.storage().persistent().get(&DataKey::BloodRequest(id))
}

/// Check if a blood request exists
#[allow(dead_code)]
pub fn blood_request_exists(env: &Env, id: u64) -> bool {
    env.storage().persistent().has(&DataKey::BloodRequest(id))
}

/// Remove a blood request (use sparingly, typically requests should be marked as cancelled)
#[allow(dead_code)]
pub fn remove_blood_request(env: &Env, id: u64) {
    env.storage().persistent().remove(&DataKey::BloodRequest(id));
}

// ========== Index Management ==========

/// Add request to hospital index
pub fn add_to_hospital_index(env: &Env, request: &BloodRequest) {
    let key = DataKey::HospitalIndex(request.hospital_id.clone());
    let mut requests: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    requests.push_back(request.id);
    env.storage().persistent().set(&key, &requests);
}

/// Add request to blood type index
pub fn add_to_blood_type_index(env: &Env, request: &BloodRequest) {
    let key = DataKey::BloodTypeIndex(request.blood_type);
    let mut requests: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    requests.push_back(request.id);
    env.storage().persistent().set(&key, &requests);
}

/// Add request to status index
pub fn add_to_status_index(env: &Env, request: &BloodRequest) {
    let key = DataKey::StatusIndex(request.status);
    let mut requests: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    requests.push_back(request.id);
    env.storage().persistent().set(&key, &requests);
}

/// Add request to urgency index
pub fn add_to_urgency_index(env: &Env, request: &BloodRequest) {
    let key = DataKey::UrgencyIndex(request.urgency);
    let mut requests: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    requests.push_back(request.id);
    env.storage().persistent().set(&key, &requests);
}

/// Remove request from status index (for status transitions)
pub fn remove_from_status_index(env: &Env, request_id: u64, status: RequestStatus) {
    let key = DataKey::StatusIndex(status);
    if let Some(requests) = env.storage().persistent().get::<DataKey, Vec<u64>>(&key) {
        // Find and remove the request ID
        let mut new_requests = Vec::new(env);
        for id in requests.iter() {
            if id != request_id {
                new_requests.push_back(id);
            }
        }
        env.storage().persistent().set(&key, &new_requests);
    }
}

/// Update status index when request status changes
pub fn update_status_index(
    env: &Env,
    request_id: u64,
    old_status: RequestStatus,
    new_status: RequestStatus,
) {
    remove_from_status_index(env, request_id, old_status);

    // Add to new status index
    let key = DataKey::StatusIndex(new_status);
    let mut requests: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    requests.push_back(request_id);
    env.storage().persistent().set(&key, &requests);
}

// ========== Index Queries ==========

/// Get all request IDs for a hospital
pub fn get_requests_by_hospital(env: &Env, hospital: &Address) -> Vec<u64> {
    let key = DataKey::HospitalIndex(hospital.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

/// Get all request IDs for a blood type
pub fn get_requests_by_blood_type(env: &Env, blood_type: BloodType) -> Vec<u64> {
    let key = DataKey::BloodTypeIndex(blood_type);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

/// Get all request IDs with a specific status
pub fn get_requests_by_status(env: &Env, status: RequestStatus) -> Vec<u64> {
    let key = DataKey::StatusIndex(status);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

/// Get all request IDs with a specific urgency level
pub fn get_requests_by_urgency(env: &Env, urgency: UrgencyLevel) -> Vec<u64> {
    let key = DataKey::UrgencyIndex(urgency);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
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
