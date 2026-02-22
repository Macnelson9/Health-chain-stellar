use crate::types::{BloodRequest, BloodType, RequestMetadata, RequestStatus, UrgencyLevel};
use crate::{RequestContract, RequestContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Address, Env, String,
};

use crate::storage;
use crate::types::{BloodType, RequestStatus, UrgencyLevel};
use crate::{RequestContract, RequestContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// ========== Test Helpers ==========

fn create_test_contract<'a>() -> (Env, Address, RequestContractClient<'a>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RequestContract, ());
    let client = RequestContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    (env, admin, client, contract_id)
}

fn setup_authorized_hospital<'a>(
    env: &Env,
    client: &RequestContractClient<'a>,
) -> Address {
    let hospital = Address::generate(env);
    client.authorize_hospital(&hospital);
    hospital
}

// ========== Initialization Tests ==========

#[test]
fn test_initialize_success() {
    let (env, admin, _client, contract_id) = create_test_contract();

    // Verify admin is set
    let stored_admin = env.as_contract(&contract_id, || storage::get_admin(&env));
    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #0)")]
fn test_initialize_already_initialized() {
    let (_env, admin, client, _contract_id) = create_test_contract();

    // Try to initialize again - should fail
    client.initialize(&admin);
}

// ========== Hospital Authorization Tests ==========

#[test]
fn test_authorize_hospital_success() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = Address::generate(&env);

    // Initially not authorized
    assert!(!client.is_hospital_authorized(&hospital));

    // Authorize
    client.authorize_hospital(&hospital);

    // Now authorized
    assert!(client.is_hospital_authorized(&hospital));
}

#[test]
fn test_revoke_hospital_success() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = Address::generate(&env);
    client.authorize_hospital(&hospital);
    assert!(client.is_hospital_authorized(&hospital));

    // Revoke
    client.revoke_hospital(&hospital);

    // No longer authorized
    assert!(!client.is_hospital_authorized(&hospital));
}

#[test]
fn test_admin_is_always_authorized() {
    let (_env, admin, client, _contract_id) = create_test_contract();

    // Admin should be authorized automatically
    assert!(client.is_hospital_authorized(&admin));
}

// ========== Create Request Tests ==========

#[test]
fn test_create_request_success() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let blood_type = BloodType::APositive;
    let quantity_ml = 900u32;
    let urgency = UrgencyLevel::Normal;
    let required_by = current_time + (7 * 86400); // 7 days
    let delivery_address = String::from_str(&env, "123 Hospital Street, City");

    let request_id = client.create_request(
        &hospital,
fn create_test_contract<'a>() -> (Env, Address, RequestContractClient<'a>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RequestContract, ());
    let client = RequestContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, admin, client, contract_id)
}

#[test]
fn test_initialize_success() {
    let (env, admin, _client, contract_id) = create_test_contract();

    // Verify admin is set
    let stored_admin = env.as_contract(&contract_id, || {
        crate::storage::get_admin(&env)
    });

    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #0)")]
fn test_initialize_already_initialized() {
    let (env, admin, client, _contract_id) = create_test_contract();

    // Try to initialize again
    client.initialize(&admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #32)")]
fn test_create_request_success() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);
    let blood_type = BloodType::OPositive;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Urgent;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (2 * 86400); // 2 days from now

    let delivery_address = String::from_str(&env, "Hospital Main Building");
    let procedure = String::from_str(&env, "Emergency Surgery");
    let notes = String::from_str(&env, "Type O+ preferred");

    // This should fail because hospital is not authorized (not the admin)
    client.create_request(
        &hospital,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #32)")]
fn test_create_request_unauthorized_hospital() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);
    let blood_type = BloodType::APositive;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Normal;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (2 * 86400);

    let delivery_address = String::from_str(&env, "Hospital Main");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // This should fail because hospital is not authorized
    client.create_request(
        &hospital,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_create_request_invalid_quantity_too_low() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::BPositive;
    let quantity_ml = 25u32; // Below minimum of 50ml
    let urgency = UrgencyLevel::Critical;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (1 * 86400);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_create_request_invalid_quantity_too_high() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::BNegative;
    let quantity_ml = 6000u32; // Above maximum of 5000ml
    let urgency = UrgencyLevel::Normal;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (5 * 86400);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #15)")]
fn test_create_request_invalid_timestamp_in_past() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::ABPositive;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Urgent;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time - 100u64; // In the past

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #15)")]
fn test_create_request_invalid_timestamp_too_far() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::ABNegative;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Normal;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (40 * 86400); // 40 days in future (max is 30)

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_create_request_empty_delivery_address() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::OPositive;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Critical;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (1 * 86400);

    let delivery_address = String::from_str(&env, ""); // Empty address
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );
}

#[test]
fn test_create_multiple_requests() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient1 = Address::generate(&env);
    let patient2 = Address::generate(&env);

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create first request
    let request_id_1 = client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient1,
        &procedure,
        &notes,
    );

    // Create second request
    let request_id_2 = client.create_request(
        &admin,
        &BloodType::ABNegative,
        &500u32,
        &UrgencyLevel::Critical,
        &(current_time + 3600),
        &delivery_address,
        &patient2,
        &procedure,
        &notes,
    );

    assert_eq!(request_id_1, 1);
    assert_eq!(request_id_2, 2);

    let req1 = client.get_request(&request_id_1);
    let req2 = client.get_request(&request_id_2);

    assert_eq!(req1.blood_type, BloodType::OPositive);
    assert_eq!(req2.blood_type, BloodType::ABNegative);
}

#[test]
fn test_update_request_status_pending_to_approved() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Update status to Approved
    client.update_request_status(&request_id, &RequestStatus::Approved);

    let request = client.get_request(&request_id);
    assert_eq!(request.status, RequestStatus::Approved);
}

#[test]
fn test_update_request_status_approved_to_fulfilled() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Normal,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Update to Approved
    client.update_request_status(&request_id, &RequestStatus::Approved);

    // Update to Fulfilled
    client.update_request_status(&request_id, &RequestStatus::Fulfilled);

    let request = client.get_request(&request_id);
    assert_eq!(request.status, RequestStatus::Fulfilled);
    assert_eq!(request.fulfilled_at, Some(current_time));
}

#[test]
#[should_panic(expected = "Error(Contract, #41)")]
fn test_update_request_status_invalid_transition() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::ABPositive,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 3600),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Try invalid transition: Pending -> Fulfilled (should be Pending -> Approved -> Fulfilled)
    client.update_request_status(&request_id, &RequestStatus::Fulfilled);
}

#[test]
#[should_panic(expected = "Error(Contract, #41)")]
fn test_update_request_status_from_terminal_state() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::ONegative,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Transition to Rejected (terminal state)
    client.update_request_status(&request_id, &RequestStatus::Rejected);

    // Try to transition from Rejected (should fail)
    client.update_request_status(&request_id, &RequestStatus::Approved);
}

#[test]
fn test_assign_blood_units() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::OPositive,
        &900u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Assign blood units
    let unit_ids = vec![&env, 1u64, 2u64];
    client.assign_blood_units(&request_id, &unit_ids);

    let request = client.get_request(&request_id);
    assert_eq!(request.assigned_units.len(), 2);
    assert_eq!(request.assigned_units.get(0).unwrap(), 1u64);
    assert_eq!(request.assigned_units.get(1).unwrap(), 2u64);
}

#[test]
#[should_panic(expected = "Error(Contract, #40)")]
fn test_request_not_found() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    // Try to get a request that doesn't exist
    client.get_request(&999u64);
}

#[test]
fn test_urgency_level_max_fulfillment_time() {
    assert_eq!(UrgencyLevel::Critical.max_fulfillment_time(), 3600); // 1 hour
    assert_eq!(UrgencyLevel::Urgent.max_fulfillment_time(), 21600); // 6 hours
    assert_eq!(UrgencyLevel::Normal.max_fulfillment_time(), 86400); // 24 hours
}

#[test]
fn test_request_status_transitions() {
    // Test valid transitions
    assert!(RequestStatus::Pending.can_transition_to(&RequestStatus::Approved));
    assert!(RequestStatus::Pending.can_transition_to(&RequestStatus::Rejected));
    assert!(RequestStatus::Pending.can_transition_to(&RequestStatus::Cancelled));

    assert!(RequestStatus::Approved.can_transition_to(&RequestStatus::Fulfilled));
    assert!(RequestStatus::Approved.can_transition_to(&RequestStatus::Cancelled));

    assert!(RequestStatus::Fulfilled.can_transition_to(&RequestStatus::Completed));

    // Test invalid transitions
    assert!(!RequestStatus::Pending.can_transition_to(&RequestStatus::Fulfilled));
    assert!(!RequestStatus::Rejected.can_transition_to(&RequestStatus::Approved));
    assert!(!RequestStatus::Completed.can_transition_to(&RequestStatus::Approved));
    assert!(!RequestStatus::Cancelled.can_transition_to(&RequestStatus::Fulfilled));
}

#[test]
fn test_request_status_is_terminal() {
    assert!(!RequestStatus::Pending.is_terminal());
    assert!(!RequestStatus::Approved.is_terminal());
    assert!(!RequestStatus::Fulfilled.is_terminal());

    assert!(RequestStatus::Completed.is_terminal());
    assert!(RequestStatus::Rejected.is_terminal());
    assert!(RequestStatus::Cancelled.is_terminal());
}

#[test]
fn test_blood_request_validate_all_blood_types() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000u64);

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);

    let blood_types = [
        BloodType::APositive,
        BloodType::ANegative,
        BloodType::BPositive,
        BloodType::BNegative,
        BloodType::ABPositive,
        BloodType::ABNegative,
        BloodType::OPositive,
        BloodType::ONegative,
    ];

    for blood_type in blood_types.iter() {
        let metadata = RequestMetadata {
            patient_id: patient.clone(),
            procedure: String::from_str(&env, "Surgery"),
            notes: String::from_str(&env, "Notes"),
        };

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital.clone(),
            blood_type: *blood_type,
            quantity_ml: 450,
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: 1000u64,
            required_by: 2000u64,
            fulfilled_at: None,
            assigned_units: soroban_sdk::vec![&env],
            delivery_address: String::from_str(&env, "Hospital"),
            metadata,
        };

        assert!(request.validate(1000u64).is_ok());
    }
}

#[test]
fn test_blood_request_is_overdue() {
    let env = Env::default();
    env.mock_all_auths();

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);

    let metadata = RequestMetadata {
        patient_id: patient,
        procedure: String::from_str(&env, "Surgery"),
        notes: String::from_str(&env, "Notes"),
    };

    let request = BloodRequest {
        id: 1,
        hospital_id: hospital,
        blood_type: BloodType::OPositive,
        quantity_ml: 450,
        urgency: UrgencyLevel::Urgent,
        status: RequestStatus::Pending,
        created_at: 1000u64,
        required_by: 2000u64,
        fulfilled_at: None,
        assigned_units: soroban_sdk::vec![&env],
        delivery_address: String::from_str(&env, "Hospital"),
        metadata,
    };

    assert!(!request.is_overdue(1500u64)); // Before deadline
    assert!(!request.is_overdue(2000u64)); // At deadline
    assert!(request.is_overdue(2001u64)); // After deadline
}

#[test]
fn test_blood_request_time_remaining() {
    let env = Env::default();
    env.mock_all_auths();

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);

    let metadata = RequestMetadata {
        patient_id: patient,
        procedure: String::from_str(&env, "Surgery"),
        notes: String::from_str(&env, "Notes"),
    };

    let request = BloodRequest {
        id: 1,
        hospital_id: hospital,
        blood_type: BloodType::BPositive,
        quantity_ml: 500,
        urgency: UrgencyLevel::Critical,
        status: RequestStatus::Pending,
        created_at: 1000u64,
        required_by: 2000u64,
        fulfilled_at: None,
        assigned_units: soroban_sdk::vec![&env],
        delivery_address: String::from_str(&env, "Hospital"),
        metadata,
    };

    assert_eq!(request.time_remaining(1000u64), 1000i64); // 1000 seconds remaining
    assert_eq!(request.time_remaining(1500u64), 500i64); // 500 seconds remaining
    assert_eq!(request.time_remaining(2000u64), 0i64); // 0 seconds remaining
    assert_eq!(request.time_remaining(2500u64), -500i64); // -500 seconds (overdue)
}

#[test]
fn test_blood_request_can_fulfill() {
    let env = Env::default();
    env.mock_all_auths();

    let hospital = Address::generate(&env);
    let patient = Address::generate(&env);

    let metadata = RequestMetadata {
        patient_id: patient,
        procedure: String::from_str(&env, "Surgery"),
        notes: String::from_str(&env, "Notes"),
    };

    let mut request = BloodRequest {
        id: 1,
        hospital_id: hospital,
        blood_type: BloodType::ABNegative,
        quantity_ml: 450,
        urgency: UrgencyLevel::Normal,
        status: RequestStatus::Approved,
        created_at: 1000u64,
        required_by: 2000u64,
        fulfilled_at: None,
        assigned_units: soroban_sdk::vec![&env],
        delivery_address: String::from_str(&env, "Hospital"),
        metadata,
    };

    // Can fulfill when Approved and not overdue
    assert!(request.can_fulfill(1500u64));

    // Cannot fulfill when overdue
    assert!(!request.can_fulfill(2001u64));

    // Cannot fulfill when not Approved
    request.status = RequestStatus::Pending;
    assert!(!request.can_fulfill(1500u64));
}


#[test]
fn test_create_request_as_admin_success() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let blood_type = BloodType::OPositive;
    let quantity_ml = 450u32;
    let urgency = UrgencyLevel::Urgent;

    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let required_by = current_time + (2 * 86400); // 2 days from now

    let delivery_address = String::from_str(&env, "Hospital Main Building");
    let procedure = String::from_str(&env, "Emergency Surgery");
    let notes = String::from_str(&env, "Type O+ preferred");

    let request_id = client.create_request(
        &admin,
        &blood_type,
        &quantity_ml,
        &urgency,
        &required_by,
        &delivery_address,
    );

    assert_eq!(request_id, 1);

    // Verify stored request
    let stored_request = client.get_request(&request_id);
    assert_eq!(stored_request.id, 1);
    assert_eq!(stored_request.hospital_id, hospital);
    assert_eq!(stored_request.blood_type, blood_type);
    assert_eq!(stored_request.quantity_ml, quantity_ml);
    assert_eq!(stored_request.urgency, urgency);
    assert_eq!(stored_request.status, RequestStatus::Pending);
    assert_eq!(stored_request.created_at, current_time);
    assert_eq!(stored_request.required_by, required_by);
    assert_eq!(stored_request.fulfilled_at, None);
    assert_eq!(stored_request.assigned_units.len(), 0);
}

#[test]
fn test_create_request_increments_id() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Create first request
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    assert_eq!(id1, 1);

    // Create second request
    let id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &required_by,
        &delivery_address,
    );
    assert_eq!(id2, 2);

    // Create third request
    let id3 = client.create_request(
        &hospital,
        &BloodType::ONegative,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 2 * 3600), // Critical needs less time
        &delivery_address,
    );
    assert_eq!(id3, 3);
}

#[test]
fn test_create_request_all_blood_types() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    let blood_types = [
        BloodType::APositive,
        BloodType::ANegative,
        BloodType::BPositive,
        BloodType::BNegative,
        BloodType::ABPositive,
        BloodType::ABNegative,
        BloodType::OPositive,
        BloodType::ONegative,
    ];

    for (i, blood_type) in blood_types.iter().enumerate() {
        let id = client.create_request(
            &hospital,
            blood_type,
            &450u32,
            &UrgencyLevel::Normal,
            &required_by,
            &delivery_address,
        );

        assert_eq!(id, (i + 1) as u64);

        let request = client.get_request(&id);
        assert_eq!(request.blood_type, *blood_type);
    }
}

#[test]
fn test_create_request_all_urgency_levels() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Critical - needs at least 1 hour
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 2 * 3600), // 2 hours
        &delivery_address,
    );
    let req1 = client.get_request(&id1);
    assert_eq!(req1.urgency, UrgencyLevel::Critical);

    // Urgent - needs at least 4 hours
    let id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 6 * 3600), // 6 hours
        &delivery_address,
    );
    let req2 = client.get_request(&id2);
    assert_eq!(req2.urgency, UrgencyLevel::Urgent);

    // Normal - needs at least 24 hours
    let id3 = client.create_request(
        &hospital,
        &BloodType::ONegative,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 48 * 3600), // 48 hours
        &delivery_address,
    );
    let req3 = client.get_request(&id3);
    assert_eq!(req3.urgency, UrgencyLevel::Normal);
}

#[test]
#[should_panic(expected = "Error(Contract, #32)")]
fn test_create_request_unauthorized_hospital() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let unauthorized_hospital = Address::generate(&env);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    client.create_request(
        &unauthorized_hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_create_request_quantity_too_low() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    client.create_request(
        &hospital,
        &BloodType::APositive,
        &50u32, // Too low (min is 100)
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_create_request_quantity_too_high() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    client.create_request(
        &hospital,
        &BloodType::APositive,
        &20000u32, // Too high (max is 10000)
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_create_request_required_by_too_soon() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    // Only 30 minutes (less than 1 hour minimum)
    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 1800),
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_create_request_required_by_too_far() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    // 60 days (more than 30 day max)
    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 60 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #19)")]
fn test_create_request_empty_delivery_address() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, ""), // Empty address
    );
}

#[test]
fn test_create_request_edge_case_quantities() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Minimum valid quantity
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &100u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let req1 = client.get_request(&id1);
    assert_eq!(req1.quantity_ml, 100);

    // Maximum valid quantity
    let id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &10000u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let req2 = client.get_request(&id2);
    assert_eq!(req2.quantity_ml, 10000);
}

// ========== Approve Request Tests ==========

#[test]
fn test_approve_request_success() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Verify initial status
    let request = client.get_request(&request_id);
    assert_eq!(request.status, RequestStatus::Pending);

    // Approve
    client.approve_request(&request_id);

    // Verify updated status
    let updated_request = client.get_request(&request_id);
    assert_eq!(updated_request.status, RequestStatus::Approved);
}

#[test]
#[should_panic(expected = "Error(Contract, #21)")]
fn test_approve_request_not_found() {
    let (_env, _admin, client, _contract_id) = create_test_contract();

    client.approve_request(&999);
}

#[test]
#[should_panic(expected = "Error(Contract, #41)")]
fn test_approve_request_already_approved() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Approve first time
    client.approve_request(&request_id);

    // Try to approve again - should fail
    client.approve_request(&request_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #22)")]
fn test_approve_request_expired() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (2 * 86400); // 2 days

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Fast forward past required_by
    env.ledger().set_timestamp(required_by + 1);

    // Try to approve expired request
    client.approve_request(&request_id);
}

// ========== Cancel Request Tests ==========

#[test]
fn test_cancel_request_by_hospital() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Cancel by hospital
    client.cancel_request(&request_id, &hospital);

    // Verify cancelled
    let request = client.get_request(&request_id);
    assert_eq!(request.status, RequestStatus::Cancelled);
}

#[test]
fn test_cancel_request_by_admin() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Cancel by admin
    client.cancel_request(&request_id, &admin);

    // Verify cancelled
    let request = client.get_request(&request_id);
    assert_eq!(request.status, RequestStatus::Cancelled);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_cancel_request_unauthorized() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);
    let other_hospital = Address::generate(&env);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Try to cancel by unauthorized party
    client.cancel_request(&request_id, &other_hospital);
}

#[test]
#[should_panic(expected = "Error(Contract, #42)")]
fn test_cancel_request_already_cancelled() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let request_id = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 7 * 86400),
        &String::from_str(&env, "123 Hospital Street"),
    );

    // Cancel first time
    client.cancel_request(&request_id, &hospital);

    // Try to cancel again
    client.cancel_request(&request_id, &hospital);
}

// ========== Query Tests ==========

#[test]
fn test_get_hospital_requests() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital1 = setup_authorized_hospital(&env, &client);
    let hospital2 = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Create requests for hospital1
    let id1 = client.create_request(
        &hospital1,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let id2 = client.create_request(
        &hospital1,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );

    // Create request for hospital2
    let _id3 = client.create_request(
        &hospital2,
        &BloodType::ONegative,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );

    // Query hospital1 requests
    let hospital1_requests = client.get_hospital_requests(&hospital1);
    assert_eq!(hospital1_requests.len(), 2);
    assert_eq!(hospital1_requests.get(0).unwrap(), id1);
    assert_eq!(hospital1_requests.get(1).unwrap(), id2);

    // Query hospital2 requests
    let hospital2_requests = client.get_hospital_requests(&hospital2);
    assert_eq!(hospital2_requests.len(), 1);
}

#[test]
fn test_get_requests_by_status() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Create requests
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );

    // Approve one request
    client.approve_request(&id1);

    // Query pending requests
    let pending_requests = client.get_requests_by_status(&RequestStatus::Pending);
    assert_eq!(pending_requests.len(), 1);
    assert_eq!(pending_requests.get(0).unwrap(), id2);

    // Query approved requests
    let approved_requests = client.get_requests_by_status(&RequestStatus::Approved);
    assert_eq!(approved_requests.len(), 1);
    assert_eq!(approved_requests.get(0).unwrap(), id1);
}

#[test]
fn test_get_requests_by_blood_type() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let required_by = current_time + (7 * 86400);
    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Create requests with different blood types
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let _id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &required_by,
        &delivery_address,
    );
    let id3 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &900u32,
        &UrgencyLevel::Urgent,
        &(current_time + 6 * 3600),
        &delivery_address,
    );

    // Query A+ requests
    let a_positive_requests = client.get_requests_by_blood_type(&BloodType::APositive);
    assert_eq!(a_positive_requests.len(), 2);
    assert_eq!(a_positive_requests.get(0).unwrap(), id1);
    assert_eq!(a_positive_requests.get(1).unwrap(), id3);
}

#[test]
fn test_get_requests_by_urgency() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "123 Hospital Street");

    // Create requests with different urgency levels
    let id1 = client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 2 * 3600),
        &delivery_address,
    );
    let _id2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 48 * 3600),
        &delivery_address,
    );
    let id3 = client.create_request(
        &hospital,
        &BloodType::ONegative,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 3 * 3600),
        &delivery_address,
    );

    // Query critical requests
    let critical_requests = client.get_requests_by_urgency(&UrgencyLevel::Critical);
    assert_eq!(critical_requests.len(), 2);
    assert_eq!(critical_requests.get(0).unwrap(), id1);
    assert_eq!(critical_requests.get(1).unwrap(), id3);
}

#[test]
#[should_panic(expected = "Error(Contract, #21)")]
fn test_get_request_not_found() {
    let (_env, _admin, client, _contract_id) = create_test_contract();

    client.get_request(&999);
}

// ========== Urgency Time Window Tests ==========

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_critical_request_insufficient_time() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    // Critical needs at least 1 hour, but we give only 30 minutes
    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 1800), // 30 minutes - too short for critical
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_urgent_request_insufficient_time() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    // Urgent needs at least 4 hours, but we give only 2 hours
    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 2 * 3600), // 2 hours - too short for urgent
        &String::from_str(&env, "123 Hospital Street"),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_normal_request_insufficient_time() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    let current_time = 1000000u64;
    env.ledger().set_timestamp(current_time);

    // Normal needs at least 24 hours, but we give only 12 hours
    client.create_request(
        &hospital,
        &BloodType::APositive,
        &450u32,
        &UrgencyLevel::Normal,
        &(current_time + 12 * 3600), // 12 hours - too short for normal
        &String::from_str(&env, "123 Hospital Street"),
        &patient,
        &procedure,
        &notes,
    );

    assert_eq!(request_id, 1);

    // Verify request was created
    let request = client.get_request(&request_id);
    assert_eq!(request.id, request_id);
    assert_eq!(request.hospital_id, admin);
    assert_eq!(request.blood_type, blood_type);
    assert_eq!(request.quantity_ml, quantity_ml);
    assert_eq!(request.urgency, urgency);
    assert_eq!(request.status, RequestStatus::Pending);
    assert_eq!(request.created_at, current_time);
    assert_eq!(request.required_by, required_by);
    assert_eq!(request.fulfilled_at, None);
    assert_eq!(request.delivery_address, delivery_address);
}

// ========== Advanced Query Function Tests ==========

#[test]
fn test_get_request_by_id_exists() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    let request_id = client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Test get_request_by_id
    let result = client.get_request_by_id(&request_id);
    assert!(result.is_some());
    
    let request = result.unwrap();
    assert_eq!(request.id, request_id);
    assert_eq!(request.blood_type, BloodType::OPositive);
}

#[test]
fn test_get_request_by_id_non_existent() {
    let (_env, _admin, client, _contract_id) = create_test_contract();

    // Test for non-existent request
    let result = client.get_request_by_id(&999u64);
    assert!(result.is_none());
}

#[test]
fn test_query_hospital_requests_all() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);
    let patient = Address::generate(&env);
    
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create multiple requests for the same hospital
    client.create_request(
        &hospital,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &hospital,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Normal,
        &(current_time + 172800),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &hospital,
        &BloodType::ABNegative,
        &350u32,
        &UrgencyLevel::Critical,
        &(current_time + 7200),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Query all requests for this hospital (no status filter)
    let requests = client.query_hospital_requests(&hospital, &None, &None, &None);
    assert_eq!(requests.len(), 3);
}

#[test]
fn test_query_hospital_requests_with_status_filter() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);
    let patient = Address::generate(&env);
    
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create requests
    let req1 = client.create_request(
        &hospital,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Urgent,
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    let req2 = client.create_request(
        &hospital,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Normal,
        &(current_time + 172800),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Approve one request
    client.approve_request(&req1);

    // Query pending requests only
    let pending_requests = client.query_hospital_requests(
        &hospital,
        &Some(RequestStatus::Pending),
        &None,
        &None,
    );
    assert_eq!(pending_requests.len(), 1);
    assert_eq!(pending_requests.get(0).unwrap().id, req2);

    // Query approved requests only
    let approved_requests = client.query_hospital_requests(
        &hospital,
        &Some(RequestStatus::Approved),
        &None,
        &None,
    );
    assert_eq!(approved_requests.len(), 1);
    assert_eq!(approved_requests.get(0).unwrap().id, req1);
}

#[test]
fn test_query_hospital_requests_pagination() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);
    let patient = Address::generate(&env);
    
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create 5 requests
    for i in 0..5 {
        client.create_request(
            &hospital,
            &BloodType::OPositive,
            &450u32,
            &UrgencyLevel::Normal,
            &(current_time + 86400 + (i * 1000)),
            &delivery_address,
            &patient,
            &procedure,
            &notes,
        );
    }

    // Test pagination: limit=2, offset=0
    let page1 = client.query_hospital_requests(&hospital, &None, &Some(2u32), &Some(0u32));
    assert_eq!(page1.len(), 2);

    // Test pagination: limit=2, offset=2
    let page2 = client.query_hospital_requests(&hospital, &None, &Some(2u32), &Some(2u32));
    assert_eq!(page2.len(), 2);

    // Test pagination: limit=2, offset=4
    let page3 = client.query_hospital_requests(&hospital, &None, &Some(2u32), &Some(4u32));
    assert_eq!(page3.len(), 1);

    // Verify different pages have different requests
    assert_ne!(page1.get(0).unwrap().id, page2.get(0).unwrap().id);
}

#[test]
fn test_query_hospital_requests_empty() {
    let (env, _admin, client, _contract_id) = create_test_contract();

    let hospital = setup_authorized_hospital(&env, &client);

    // Query requests for hospital with no requests
    let requests = client.query_hospital_requests(&hospital, &None, &None, &None);
    assert_eq!(requests.len(), 0);
}

#[test]
fn test_query_pending_requests_sorted_by_urgency() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create requests with different urgencies (in non-sorted order)
    client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Normal,  // Priority 1
        &(current_time + 86400),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &admin,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Critical,  // Priority 3
        &(current_time + 7200),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &admin,
        &BloodType::ABNegative,
        &350u32,
        &UrgencyLevel::Urgent,  // Priority 2
        &(current_time + 21600),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Query pending requests (should be sorted by urgency)
    let requests = client.query_pending_requests(&None, &None);
    assert_eq!(requests.len(), 3);

    // Verify sorting: Critical > Urgent > Normal
    assert_eq!(requests.get(0).unwrap().urgency, UrgencyLevel::Critical);
    assert_eq!(requests.get(1).unwrap().urgency, UrgencyLevel::Urgent);
    assert_eq!(requests.get(2).unwrap().urgency, UrgencyLevel::Normal);
}

#[test]
fn test_query_pending_requests_pagination() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create 4 pending requests
    for _ in 0..4 {
        client.create_request(
            &admin,
            &BloodType::OPositive,
            &450u32,
            &UrgencyLevel::Urgent,
            &(current_time + 86400),
            &delivery_address,
            &patient,
            &procedure,
            &notes,
        );
    }

    // Test pagination
    let page1 = client.query_pending_requests(&Some(2u32), &Some(0u32));
    assert_eq!(page1.len(), 2);

    let page2 = client.query_pending_requests(&Some(2u32), &Some(2u32));
    assert_eq!(page2.len(), 2);

    // Verify different IDs
    assert_ne!(page1.get(0).unwrap().id, page2.get(0).unwrap().id);
}

#[test]
fn test_query_requests_by_date_range() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create requests at different times
    env.ledger().set_timestamp(1000u64);
    client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &2000u64,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    env.ledger().set_timestamp(5000u64);
    client.create_request(
        &admin,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Urgent,
        &6000u64,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    env.ledger().set_timestamp(10000u64);
    client.create_request(
        &admin,
        &BloodType::ABNegative,
        &350u32,
        &UrgencyLevel::Critical,
        &11000u64,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Query requests created between 1000 and 6000
    let requests = client.query_requests_by_date_range(
        &1000u64,
        &6000u64,
        &None,
        &None,
        &None,
    );
    assert_eq!(requests.len(), 2);

    // Query requests created between 5000 and 10000
    let requests2 = client.query_requests_by_date_range(
        &5000u64,
        &10000u64,
        &None,
        &None,
        &None,
    );
    assert_eq!(requests2.len(), 2);

    // Query with narrow range
    let requests3 = client.query_requests_by_date_range(
        &4000u64,
        &6000u64,
        &None,
        &None,
        &None,
    );
    assert_eq!(requests3.len(), 1);
}

#[test]
fn test_query_requests_by_date_range_with_status_filter() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create requests
    env.ledger().set_timestamp(1000u64);
    let req1 = client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Normal,
        &2000u64,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    env.ledger().set_timestamp(2000u64);
    client.create_request(
        &admin,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Urgent,
        &3000u64,
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Approve one request
    client.approve_request(&req1);

    // Query approved requests in date range
    let requests = client.query_requests_by_date_range(
        &1000u64,
        &2000u64,
        &Some(RequestStatus::Approved),
        &None,
        &None,
    );
    assert_eq!(requests.len(), 1);
    assert_eq!(requests.get(0).unwrap().status, RequestStatus::Approved);

    // Query pending requests in date range
    let pending = client.query_requests_by_date_range(
        &1000u64,
        &3000u64,
        &Some(RequestStatus::Pending),
        &None,
        &None,
    );
    assert_eq!(pending.len(), 1);
    assert_eq!(pending.get(0).unwrap().status, RequestStatus::Pending);
}

#[test]
fn test_query_requests_by_urgency_and_status() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create requests with different urgencies
    let req1 = client.create_request(
        &admin,
        &BloodType::OPositive,
        &450u32,
        &UrgencyLevel::Critical,
        &(current_time + 7200),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &admin,
        &BloodType::BPositive,
        &500u32,
        &UrgencyLevel::Critical,
        &(current_time + 7200),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    client.create_request(
        &admin,
        &BloodType::ABNegative,
        &350u32,
        &UrgencyLevel::Urgent,
        &(current_time + 21600),
        &delivery_address,
        &patient,
        &procedure,
        &notes,
    );

    // Approve one critical request
    client.approve_request(&req1);

    // Query all critical requests (no status filter)
    let critical_all = client.query_requests_by_urgency_and_status(
        &UrgencyLevel::Critical,
        &None,
        &None,
        &None,
    );
    assert_eq!(critical_all.len(), 2);

    // Query critical pending requests only
    let critical_pending = client.query_requests_by_urgency_and_status(
        &UrgencyLevel::Critical,
        &Some(RequestStatus::Pending),
        &None,
        &None,
    );
    assert_eq!(critical_pending.len(), 1);

    // Query critical approved requests only
    let critical_approved = client.query_requests_by_urgency_and_status(
        &UrgencyLevel::Critical,
        &Some(RequestStatus::Approved),
        &None,
        &None,
    );
    assert_eq!(critical_approved.len(), 1);
}

#[test]
fn test_pagination_edge_cases() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create 3 requests
    for _ in 0..3 {
        client.create_request(
            &admin,
            &BloodType::OPositive,
            &450u32,
            &UrgencyLevel::Normal,
            &(current_time + 86400),
            &delivery_address,
            &patient,
            &procedure,
            &notes,
        );
    }

    // Test: offset beyond length returns empty
    let result = client.query_pending_requests(&Some(10u32), &Some(10u32));
    assert_eq!(result.len(), 0);

    // Test: limit =0 returns no results (capped to 0)
    let result2 = client.query_pending_requests(&Some(0u32), &Some(0u32));
    assert_eq!(result2.len(), 0);

    // Test: large limit returns all available
    let result3 = client.query_pending_requests(&Some(100u32), &Some(0u32));
    assert_eq!(result3.len(), 3);
}

#[test]
fn test_max_query_limit_enforcement() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let patient = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let delivery_address = String::from_str(&env, "Hospital");
    let procedure = String::from_str(&env, "Surgery");
    let notes = String::from_str(&env, "Notes");

    // Create 10 requests
    for _ in 0..10 {
        client.create_request(
            &admin,
            &BloodType::OPositive,
            &450u32,
            &UrgencyLevel::Normal,
            &(current_time + 86400),
            &delivery_address,
            &patient,
            &procedure,
            &notes,
        );
    }

    // Request with limit > MAX_QUERY_LIMIT should be capped to MAX_QUERY_LIMIT
    // MAX_QUERY_LIMIT is 200, so requesting 300 should return at most 10 (all available)
    let result = client.query_pending_requests(&Some(300u32), &Some(0u32));
    assert_eq!(result.len(), 10);
}

