use crate::types::{BloodRequest, BloodType, RequestMetadata, RequestStatus, UrgencyLevel};
use crate::{RequestContract, RequestContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Address, Env, String,
};

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
