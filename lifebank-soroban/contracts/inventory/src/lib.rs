#![no_std]

mod error;
mod events;
mod storage;
mod types;
mod validation;

use crate::error::ContractError;
use crate::types::{BloodStatus, BloodType, BloodUnit, DataKey};

use soroban_sdk::{contract, contractimpl, Address, Env, Map};
#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {
    /// Initialize the inventory contract
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `admin` - Admin address who can authorize blood banks
    ///
    /// # Errors
    /// - `AlreadyInitialized`: Contract has already been initialized
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        admin.require_auth();

        // Check if already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        // Set admin
        storage::set_admin(&env, &admin);

        Ok(())
    }

    /// Register a new blood donation into the inventory
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `bank_id` - Blood bank's address (must be authorized)
    /// * `blood_type` - Type of blood (A+, A-, B+, B-, AB+, AB-, O+, O-)
    /// * `quantity_ml` - Quantity in milliliters (100-600ml)
    /// * `expiration_timestamp` - Unix timestamp when blood expires
    /// * `donor_id` - Optional donor address (None for anonymous)
    ///
    /// # Returns
    /// Unique ID of the registered blood unit
    ///
    /// # Errors
    /// - `NotInitialized`: Contract not initialized
    /// - `NotAuthorizedBloodBank`: Bank is not authorized
    /// - `InvalidQuantity`: Quantity outside acceptable range
    /// - `InvalidExpiration`: Expiration date is invalid
    ///
    /// # Events
    /// Emits `BloodRegistered` event with all blood unit details
    pub fn register_blood(
        env: Env,
        bank_id: Address,
        blood_type: BloodType,
        quantity_ml: u32,
        expiration_timestamp: u64,
        donor_id: Option<Address>,
    ) -> Result<u64, ContractError> {
        // 1. Verify bank authentication
        bank_id.require_auth();

        // 2. Check contract is initialized
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::NotInitialized);
        }

        // 3. Verify bank is authorized
        if !storage::is_authorized_bank(&env, &bank_id) {
            return Err(ContractError::NotAuthorizedBloodBank);
        }

        // 4. Validate input parameters
        validation::validate_blood_registration(&env, quantity_ml, expiration_timestamp)?;
        validation::validate_minimum_shelf_life(&env, expiration_timestamp)?;

        // 5. Generate unique blood unit ID
        let blood_unit_id = storage::increment_blood_unit_id(&env);

        // 6. Create blood unit struct
        let current_time = env.ledger().timestamp();
        let blood_unit = BloodUnit {
            id: blood_unit_id,
            blood_type,
            quantity_ml,
            bank_id: bank_id.clone(),
            donor_id: donor_id.clone(),
            donation_timestamp: current_time,
            expiration_timestamp,
            status: BloodStatus::Available,
            metadata: Map::new(&env),
        };

        // 7. Validate the complete blood unit
        blood_unit.validate(current_time)?;

        // 8. Store blood unit
        storage::set_blood_unit(&env, &blood_unit);

        // 9. Update indexes for efficient querying
        storage::add_to_blood_type_index(&env, &blood_unit);
        storage::add_to_bank_index(&env, &blood_unit);
        storage::add_to_status_index(&env, &blood_unit);
        storage::add_to_donor_index(&env, &blood_unit);

        // 10. Emit event
        events::emit_blood_registered(
            &env,
            blood_unit_id,
            &bank_id,
            blood_type,
            quantity_ml,
            expiration_timestamp,
        );

        // 11. Return blood unit ID
        Ok(blood_unit_id)
    }

    /// Get blood unit details by ID
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `blood_unit_id` - ID of the blood unit to retrieve
    ///
    /// # Returns
    /// Blood unit details
    ///
    /// # Errors
    /// - `NotFound`: Blood unit with given ID doesn't exist
    pub fn get_blood_unit(env: Env, blood_unit_id: u64) -> Result<BloodUnit, ContractError> {
        storage::get_blood_unit(&env, blood_unit_id).ok_or(ContractError::NotFound)
    }
}

#[cfg(test)]
mod test;
