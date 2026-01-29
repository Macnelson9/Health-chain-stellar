#![no_std]

mod error;
mod events;
mod storage;
mod types;
mod validation;

use crate::error::ContractError;
use crate::types::{BloodRequest, BloodType, RequestMetadata, RequestStatus, UrgencyLevel};
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct RequestContract;

#[contractimpl]
impl RequestContract {
    /// Initialize the request contract
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `admin` - Admin address who can authorize hospitals and blood banks
    ///
    /// # Errors
    /// - `AlreadyInitialized`: Contract has already been initialized
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        admin.require_auth();

        // Check if already initialized
        if env.storage().instance().has(&types::DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Create a new blood request
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `hospital_id` - Hospital requesting blood (must be authorized)
    /// * `blood_type` - Type of blood requested
    /// * `quantity_ml` - Quantity in milliliters (50-5000ml)
    /// * `urgency` - Urgency level (Critical, Urgent, Normal)
    /// * `required_by` - Unix timestamp when blood is required
    /// * `delivery_address` - Address where blood should be delivered
    /// * `patient_id` - Patient address/identifier
    /// * `procedure` - Medical procedure requiring blood
    /// * `notes` - Additional notes or requirements
    ///
    /// # Returns
    /// Unique ID of the created request
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `NotAuthorizedHospital`: Hospital is not authorized
    /// - `InvalidQuantity`: Quantity outside acceptable range
    /// - `InvalidTimestamp`: Required_by timestamp is invalid
    /// - `InvalidInput`: Delivery address is empty
    pub fn create_request(
        env: Env,
        hospital_id: Address,
        blood_type: BloodType,
        quantity_ml: u32,
        urgency: UrgencyLevel,
        required_by: u64,
        delivery_address: String,
        patient_id: Address,
        procedure: String,
        notes: String,
    ) -> Result<u64, ContractError> {
        // 1. Verify hospital authentication
        hospital_id.require_auth();

        // 2. Check contract is initialized
        if !env.storage().instance().has(&types::DataKey::Admin) {
            return Err(ContractError::NotInitialized);
        }

        // 3. Verify hospital is authorized
        if !storage::is_authorized_hospital(&env, &hospital_id) {
            return Err(ContractError::NotAuthorizedHospital);
        }

        // 4. Validate request parameters
        validation::validate_request_creation(&env, quantity_ml, required_by)?;
        validation::validate_delivery_address(&delivery_address)?;
        validation::validate_blood_type(&blood_type)?;

        // 5. Generate request ID
        let request_id = storage::increment_request_id(&env);
        let current_time = env.ledger().timestamp();

        // 6. Create request
        let metadata = RequestMetadata {
            patient_id,
            procedure,
            notes,
        };

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
            assigned_units: soroban_sdk::vec![&env],
            delivery_address,
            metadata,
        };

        // 7. Validate request
        request.validate(current_time)?;

        // 8. Store request
        storage::set_blood_request(&env, &request);

        // 9. Emit event
        events::emit_request_created(
            &env,
            request_id,
            &hospital_id,
            blood_type,
            quantity_ml,
            urgency,
            required_by,
        );

        Ok(request_id)
    }

    /// Update request status
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of request to update
    /// * `new_status` - New status for the request
    ///
    /// # Errors
    /// - `RequestNotFound`: Request does not exist
    /// - `InvalidStatusTransition`: Status transition is not allowed
    /// - `Unauthorized`: Caller is not authorized
    pub fn update_request_status(
        env: Env,
        request_id: u64,
        new_status: RequestStatus,
    ) -> Result<(), ContractError> {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Get existing request
        let mut request = storage::get_blood_request(&env, request_id)
            .ok_or(ContractError::RequestNotFound)?;

        // Validate status transition
        if !request.status.can_transition_to(&new_status) {
            return Err(ContractError::InvalidStatusTransition);
        }

        let old_status = request.status;
        request.status = new_status;

        // Set fulfilled_at if transitioning to Fulfilled
        if new_status == RequestStatus::Fulfilled {
            request.fulfilled_at = Some(env.ledger().timestamp());
        }

        // Store updated request
        storage::set_blood_request(&env, &request);

        // Emit event
        events::emit_request_status_changed(&env, request_id, old_status, new_status);

        Ok(())
    }

    /// Assign blood units to a request
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of request
    /// * `unit_ids` - Vector of blood unit IDs to assign
    ///
    /// # Errors
    /// - `RequestNotFound`: Request does not exist
    /// - `Unauthorized`: Caller is not authorized
    pub fn assign_blood_units(
        env: Env,
        request_id: u64,
        unit_ids: soroban_sdk::Vec<u64>,
    ) -> Result<(), ContractError> {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Get existing request
        let mut request = storage::get_blood_request(&env, request_id)
            .ok_or(ContractError::RequestNotFound)?;

        // Assign units
        request.assigned_units = unit_ids.clone();

        // Store updated request
        storage::set_blood_request(&env, &request);

        // Emit event
        events::emit_units_assigned(&env, request_id, unit_ids);

        Ok(())
    }

    /// Get a blood request by ID
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `request_id` - ID of request to retrieve
    ///
    /// # Returns
    /// The blood request if found
    ///
    /// # Errors
    /// - `RequestNotFound`: Request does not exist
    pub fn get_request(env: Env, request_id: u64) -> Result<BloodRequest, ContractError> {
        storage::get_blood_request(&env, request_id).ok_or(ContractError::RequestNotFound)
    }
}

#[cfg(test)]
mod test;
