use crate::types::{
    BloodType, RequestCreatedEvent, RequestStatus, RequestStatusChangedEvent, UnitsAssignedEvent,
    UrgencyLevel,
};
use soroban_sdk::{Address, Env, Symbol, Vec};

/// Emit a RequestCreated event when a new blood request is submitted
///
/// # Arguments
/// * `env` - Contract environment
/// * `request_id` - Unique ID of the created request
/// * `hospital_id` - Hospital that created the request
/// * `blood_type` - Requested blood type
/// * `quantity_ml` - Quantity requested in milliliters
/// * `urgency` - Urgency level of the request
/// * `required_by` - Timestamp when blood is required
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

/// Emit a RequestStatusChanged event when request status is updated
///
/// # Arguments
/// * `env` - Contract environment
/// * `request_id` - ID of the request
/// * `old_status` - Previous status
/// * `new_status` - New status
pub fn emit_status_changed(
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
        .publish((Symbol::new(env, "status_changed"),), event);
}

/// Emit an UnitsAssigned event when blood units are assigned to a request
///
/// # Arguments
/// * `env` - Contract environment
/// * `request_id` - ID of the request
/// * `unit_ids` - IDs of the assigned blood units
/// * `total_quantity_ml` - Total quantity assigned in milliliters
#[allow(dead_code)]
pub fn emit_units_assigned(
    env: &Env,
    request_id: u64,
    unit_ids: Vec<u64>,
    total_quantity_ml: u32,
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
        unit_ids,
        total_quantity_ml,
        assigned_units,
        assigned_at,
    };

    env.events()
        .publish((Symbol::new(env, "units_assigned"),), event);
}

/// Emit a request approved event
///
/// This is a simplified event using just the request ID
pub fn emit_request_approved(env: &Env, request_id: u64) {
    emit_status_changed(env, request_id, RequestStatus::Pending, RequestStatus::Approved);
}

/// Emit a request cancelled event
///
/// # Arguments
/// * `env` - Contract environment
/// * `request_id` - ID of the cancelled request
/// * `previous_status` - Status before cancellation
pub fn emit_request_cancelled(env: &Env, request_id: u64, previous_status: RequestStatus) {
    emit_status_changed(env, request_id, previous_status, RequestStatus::Cancelled);
}

/// Emit a request expired event
///
/// # Arguments
/// * `env` - Contract environment
/// * `request_id` - ID of the expired request
/// * `previous_status` - Status before expiration
#[allow(dead_code)]
pub fn emit_request_expired(env: &Env, request_id: u64, previous_status: RequestStatus) {
    emit_status_changed(env, request_id, previous_status, RequestStatus::Expired);
}

/// Emit a request completed event
#[allow(dead_code)]
pub fn emit_request_completed(env: &Env, request_id: u64) {
    emit_status_changed(
        env,
        request_id,
        RequestStatus::InDelivery,
        RequestStatus::Completed,
    );
}
