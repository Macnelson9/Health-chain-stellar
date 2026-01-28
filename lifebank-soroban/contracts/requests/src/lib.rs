#![no_std]

mod error;
mod events;
mod storage;
mod types;
mod validation;

pub use crate::error::ContractError;
pub use crate::types::{
    BloodRequest, BloodType, DataKey, RequestCreatedEvent, RequestStatus,
    RequestStatusChangedEvent, UnitsAssignedEvent, UrgencyLevel,
};

use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};

#[contract]
pub struct RequestContract;

#[contractimpl]
impl RequestContract {
    /// Initialize the request contract
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `admin` - Admin address who can manage hospitals and approve requests
    ///
    /// # Errors
    /// - `AlreadyInitialized`: Contract has already been initialized
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        admin.require_auth();

        // Check if already initialized
        if storage::is_initialized(&env) {
            return Err(ContractError::AlreadyInitialized);
        }

        // Set admin
        storage::set_admin(&env, &admin);

        Ok(())
    }

    /// Authorize a hospital to create blood requests
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital` - Hospital address to authorize
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `Unauthorized`: Caller is not admin
    pub fn authorize_hospital(env: Env, hospital: Address) -> Result<(), ContractError> {
        // Check contract is initialized
        if !storage::is_initialized(&env) {
            return Err(ContractError::NotInitialized);
        }

        // Only admin can authorize hospitals
        let admin = storage::get_admin(&env);
        admin.require_auth();

        storage::authorize_hospital(&env, &hospital);

        Ok(())
    }

    /// Revoke hospital authorization
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital` - Hospital address to revoke
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `Unauthorized`: Caller is not admin
    pub fn revoke_hospital(env: Env, hospital: Address) -> Result<(), ContractError> {
        if !storage::is_initialized(&env) {
            return Err(ContractError::NotInitialized);
        }

        let admin = storage::get_admin(&env);
        admin.require_auth();

        storage::revoke_hospital(&env, &hospital);

        Ok(())
    }

    /// Create a new blood request
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital_id` - Hospital submitting the request (must be authorized)
    /// * `blood_type` - Type of blood needed
    /// * `quantity_ml` - Quantity needed in milliliters (100-10000ml)
    /// * `urgency` - Urgency level (Critical, Urgent, Normal)
    /// * `required_by` - Unix timestamp when blood is required
    /// * `delivery_address` - Physical address for delivery
    ///
    /// # Returns
    /// Unique ID of the created request
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `NotAuthorizedHospital`: Hospital is not authorized
    /// - `InvalidQuantity`: Quantity outside acceptable range
    /// - `InvalidRequiredBy`: Invalid required_by timestamp
    /// - `InvalidDeliveryAddress`: Empty delivery address
    ///
    /// # Events
    /// Emits `RequestCreated` event with all request details
    pub fn create_request(
        env: Env,
        hospital_id: Address,
        blood_type: BloodType,
        quantity_ml: u32,
        urgency: UrgencyLevel,
        required_by: u64,
        delivery_address: String,
    ) -> Result<u64, ContractError> {
        // 1. Verify hospital authentication
        hospital_id.require_auth();

        // 2. Check contract is initialized
        if !storage::is_initialized(&env) {
            return Err(ContractError::NotInitialized);
        }

        // 3. Verify hospital is authorized
        if !storage::is_authorized_hospital(&env, &hospital_id) {
            return Err(ContractError::NotAuthorizedHospital);
        }

        // 4. Validate input parameters
        validation::validate_request_creation(&env, quantity_ml, required_by, &delivery_address)?;

        // 5. Validate urgency-specific time window
        validation::validate_urgency_time_window(&env, required_by, urgency.priority_weight())?;

        // 6. Generate unique request ID
        let request_id = storage::increment_request_id(&env);

        // 7. Create blood request struct
        let current_time = env.ledger().timestamp();
        let request = BloodRequest {
            id: request_id,
            hospital_id: hospital_id.clone(),
            blood_type,
            quantity_ml,
            urgency,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by,
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address,
            metadata: Map::new(&env),
        };

        // 8. Validate the complete request
        request.validate(current_time)?;

        // 9. Store request
        storage::set_blood_request(&env, &request);

        // 10. Update indexes for efficient querying
        storage::add_to_hospital_index(&env, &request);
        storage::add_to_blood_type_index(&env, &request);
        storage::add_to_status_index(&env, &request);
        storage::add_to_urgency_index(&env, &request);

        // 11. Emit event
        events::emit_request_created(
            &env,
            request_id,
            &hospital_id,
            blood_type,
            quantity_ml,
            urgency,
            required_by,
        );

        // 12. Return request ID
        Ok(request_id)
    }

    /// Get blood request details by ID
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of the request to retrieve
    ///
    /// # Returns
    /// Blood request details
    ///
    /// # Errors
    /// - `NotFound`: Request with given ID doesn't exist
    pub fn get_request(env: Env, request_id: u64) -> Result<BloodRequest, ContractError> {
        storage::get_blood_request(&env, request_id).ok_or(ContractError::NotFound)
    }

    /// Approve a pending blood request
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of the request to approve
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `NotFound`: Request doesn't exist
    /// - `InvalidStatusTransition`: Request is not in Pending status
    pub fn approve_request(env: Env, request_id: u64) -> Result<(), ContractError> {
        if !storage::is_initialized(&env) {
            return Err(ContractError::NotInitialized);
        }

        // Only admin can approve
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Get request
        let mut request =
            storage::get_blood_request(&env, request_id).ok_or(ContractError::NotFound)?;

        // Check valid transition
        if !request.status.can_transition_to(&RequestStatus::Approved) {
            return Err(ContractError::InvalidStatusTransition);
        }

        // Check not expired
        validation::validate_not_expired(&env, request.required_by)?;

        // Update status
        let old_status = request.status;
        request.status = RequestStatus::Approved;

        // Save and update index
        storage::set_blood_request(&env, &request);
        storage::update_status_index(&env, request_id, old_status, RequestStatus::Approved);

        // Emit event
        events::emit_request_approved(&env, request_id);

        Ok(())
    }

    /// Cancel a blood request
    ///
    /// Can be called by the hospital that created the request or by admin
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of the request to cancel
    /// * `caller` - Address of the caller (hospital or admin)
    ///
    /// # Errors
    /// - `NotFound`: Request doesn't exist
    /// - `CannotCancelRequest`: Request cannot be cancelled in current state
    /// - `Unauthorized`: Caller is not hospital owner or admin
    pub fn cancel_request(env: Env, request_id: u64, caller: Address) -> Result<(), ContractError> {
        caller.require_auth();

        if !storage::is_initialized(&env) {
            return Err(ContractError::NotInitialized);
        }

        // Get request
        let mut request =
            storage::get_blood_request(&env, request_id).ok_or(ContractError::NotFound)?;

        // Check caller is authorized (hospital owner or admin)
        let admin = storage::get_admin(&env);
        if caller != request.hospital_id && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        // Check can cancel
        if !request.status.can_cancel() {
            return Err(ContractError::CannotCancelRequest);
        }

        // Update status
        let old_status = request.status;
        request.status = RequestStatus::Cancelled;

        // Save and update index
        storage::set_blood_request(&env, &request);
        storage::update_status_index(&env, request_id, old_status, RequestStatus::Cancelled);

        // Emit event
        events::emit_request_cancelled(&env, request_id, old_status);

        Ok(())
    }

    /// Get all requests for a specific hospital
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital` - Hospital address
    ///
    /// # Returns
    /// Vector of request IDs
    pub fn get_hospital_requests(env: Env, hospital: Address) -> Vec<u64> {
        storage::get_requests_by_hospital(&env, &hospital)
    }

    /// Get all requests with a specific status
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `status` - Request status to filter by
    ///
    /// # Returns
    /// Vector of request IDs
    pub fn get_requests_by_status(env: Env, status: RequestStatus) -> Vec<u64> {
        storage::get_requests_by_status(&env, status)
    }

    /// Get all requests for a specific blood type
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `blood_type` - Blood type to filter by
    ///
    /// # Returns
    /// Vector of request IDs
    pub fn get_requests_by_blood_type(env: Env, blood_type: BloodType) -> Vec<u64> {
        storage::get_requests_by_blood_type(&env, blood_type)
    }

    /// Get all requests with a specific urgency level
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `urgency` - Urgency level to filter by
    ///
    /// # Returns
    /// Vector of request IDs
    pub fn get_requests_by_urgency(env: Env, urgency: UrgencyLevel) -> Vec<u64> {
        storage::get_requests_by_urgency(&env, urgency)
    }

    /// Check if a hospital is authorized
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital` - Hospital address to check
    ///
    /// # Returns
    /// true if authorized, false otherwise
    pub fn is_hospital_authorized(env: Env, hospital: Address) -> bool {
        storage::is_authorized_hospital(&env, &hospital)
    }
}

#[cfg(test)]
mod test;
