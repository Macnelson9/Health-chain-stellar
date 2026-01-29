use crate::types::{BloodType, RequestStatus, UrgencyLevel};
use soroban_sdk::{Address, Env, Symbol};

/// Event emitted when a blood request is created
#[soroban_sdk::contracttype]
#[derive(Clone)]
pub struct RequestCreatedEvent {
    pub request_id: u64,
    pub hospital_id: Address,
    pub blood_type: BloodType,
    pub quantity_ml: u32,
    pub urgency: UrgencyLevel,
    pub required_by: u64,
    pub created_at: u64,
}

/// Event emitted when a request status changes
#[soroban_sdk::contracttype]
#[derive(Clone)]
pub struct RequestStatusChangedEvent {
    pub request_id: u64,
    pub old_status: RequestStatus,
    pub new_status: RequestStatus,
    pub changed_at: u64,
}

/// Event emitted when blood units are assigned to a request
#[soroban_sdk::contracttype]
#[derive(Clone)]
pub struct UnitsAssignedEvent {
    pub request_id: u64,
    pub assigned_units: soroban_sdk::Vec<u64>,
    pub assigned_at: u64,
}

/// Emit a RequestCreated event
pub fn emit_request_created(
    env: &Env,
    request_id: u64,
    hospital_id: &Address,
    blood_type: BloodType,
    quantity_ml: u32,
    urgency: UrgencyLevel,
    required_by: u64,
) {
    let created_at = env.ledger().timestamp();

    let event = RequestCreatedEvent {
        request_id,
        hospital_id: hospital_id.clone(),
        blood_type,
        quantity_ml,
        urgency,
        required_by,
        created_at,
    };

    env.events()
        .publish((Symbol::new(env, "request_created"),), event);
}

/// Emit a RequestStatusChanged event
pub fn emit_request_status_changed(
    env: &Env,
    request_id: u64,
    old_status: RequestStatus,
    new_status: RequestStatus,
) {
    let changed_at = env.ledger().timestamp();

    let event = RequestStatusChangedEvent {
        request_id,
        old_status,
        new_status,
        changed_at,
    };

    env.events()
        .publish((Symbol::new(env, "request_status_changed"),), event);
}

/// Emit an UnitsAssigned event
pub fn emit_units_assigned(
    env: &Env,
    request_id: u64,
    assigned_units: soroban_sdk::Vec<u64>,
) {
    let assigned_at = env.ledger().timestamp();

    let event = UnitsAssignedEvent {
        request_id,
        assigned_units,
        assigned_at,
    };

    env.events()
        .publish((Symbol::new(env, "units_assigned"),), event);
}
