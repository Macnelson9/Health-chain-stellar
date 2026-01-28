#![cfg(test)]

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
    );
}
