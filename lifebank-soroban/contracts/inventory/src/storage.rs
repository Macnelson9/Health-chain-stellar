use crate::types::{BloodUnit, DataKey};
use soroban_sdk::{Address, Env, Vec};

/// Maximum expiration time (42 days for whole blood)
pub const MAX_EXPIRATION_DAYS: u64 = 42;
pub const SECONDS_PER_DAY: u64 = 86400;

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

/// Check if an address is authorized as a blood bank
pub fn is_authorized_bank(env: &Env, bank: &Address) -> bool {
    let admin = get_admin(env);
    bank == &admin
}

/// Get the current blood unit counter
pub fn get_blood_unit_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::BloodUnitCounter)
        .unwrap_or(0)
}

/// Increment and return the next blood unit ID
pub fn increment_blood_unit_id(env: &Env) -> u64 {
    let current = get_blood_unit_counter(env);
    let next_id = current + 1;
    env.storage()
        .instance()
        .set(&DataKey::BloodUnitCounter, &next_id);
    next_id
}

/// Store a blood unit
pub fn set_blood_unit(env: &Env, blood_unit: &BloodUnit) {
    env.storage()
        .persistent()
        .set(&DataKey::BloodUnit(blood_unit.id), blood_unit);
}

/// Get a blood unit by ID
pub fn get_blood_unit(env: &Env, id: u64) -> Option<BloodUnit> {
    env.storage().persistent().get(&DataKey::BloodUnit(id))
}

/// Check if a blood unit exists
pub fn blood_unit_exists(env: &Env, id: u64) -> bool {
    env.storage().persistent().has(&DataKey::BloodUnit(id))
}

/// Add blood unit to blood type index
pub fn add_to_blood_type_index(env: &Env, blood_unit: &BloodUnit) {
    let key = DataKey::BloodTypeIndex(blood_unit.blood_type);
    let mut units: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    units.push_back(blood_unit.id);
    env.storage().persistent().set(&key, &units);
}

/// Add blood unit to bank index
pub fn add_to_bank_index(env: &Env, blood_unit: &BloodUnit) {
    let key = DataKey::BankIndex(blood_unit.bank_id.clone());
    let mut units: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    units.push_back(blood_unit.id);
    env.storage().persistent().set(&key, &units);
}

/// Add blood unit to status index
pub fn add_to_status_index(env: &Env, blood_unit: &BloodUnit) {
    let key = DataKey::StatusIndex(blood_unit.status);
    let mut units: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    units.push_back(blood_unit.id);
    env.storage().persistent().set(&key, &units);
}

/// Add blood unit to donor index (if donor_id exists)
pub fn add_to_donor_index(env: &Env, blood_unit: &BloodUnit) {
    if let Some(donor) = &blood_unit.donor_id {
        let key = DataKey::DonorIndex(donor.clone());
        let mut units: Vec<u64> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(env));

        units.push_back(blood_unit.id);
        env.storage().persistent().set(&key, &units);
    }
}
