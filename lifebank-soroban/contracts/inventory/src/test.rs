use crate::storage;
use crate::types::{BloodRegisteredEvent, BloodStatus, BloodType};
use crate::{InventoryContract, InventoryContractClient};
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    vec, Address, Env, IntoVal, Symbol, Vec as SdkVec,
};

fn create_test_contract<'a>() -> (Env, Address, InventoryContractClient<'a>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(InventoryContract, ());
    let client = InventoryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    (env, admin, client, contract_id)
}

#[test]
fn test_initialize_success() {
    let (env, admin, client, contract_id) = create_test_contract();

    // Verify admin is set
    let stored_admin = env.as_contract(&contract_id, || storage::get_admin(&env));

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
fn test_register_blood_success() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone(); // Admin is authorized by default
    let blood_type = BloodType::APositive;
    let quantity_ml = 450u32;

    // Set current time and calculate expiration (30 days from now)
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let expiration = current_time + (30 * 86400);

    let donor = Address::generate(&env);

    let blood_unit_id = client.register_blood(
        &bank,
        &blood_type,
        &quantity_ml,
        &expiration,
        &Some(donor.clone()),
    );

    assert_eq!(blood_unit_id, 1);

    // Verify blood unit was stored
    let stored_unit = client.get_blood_unit(&blood_unit_id);
    assert_eq!(stored_unit.id, 1);
    assert_eq!(stored_unit.blood_type, blood_type);
    assert_eq!(stored_unit.quantity_ml, quantity_ml);
    assert_eq!(stored_unit.bank_id, bank);
    assert_eq!(stored_unit.donor_id, Some(donor));
    assert_eq!(stored_unit.donation_timestamp, current_time);
    assert_eq!(stored_unit.expiration_timestamp, expiration);
    assert_eq!(stored_unit.status, BloodStatus::Available);
}

#[test]
fn test_register_blood_anonymous_donor() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    let blood_unit_id = client.register_blood(
        &bank,
        &BloodType::ONegative,
        &450u32,
        &(current_time + 30 * 86400),
        &None, // Anonymous donor
    );

    let stored_unit = client.get_blood_unit(&blood_unit_id);
    assert_eq!(stored_unit.donor_id, None);
}

#[test]
fn test_register_blood_increments_id() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let expiration = current_time + (30 * 86400);

    // Register first unit
    let id1 = client.register_blood(&bank, &BloodType::APositive, &450u32, &expiration, &None);
    assert_eq!(id1, 1);

    // Register second unit
    let id2 = client.register_blood(&bank, &BloodType::BPositive, &450u32, &expiration, &None);
    assert_eq!(id2, 2);

    // Register third unit
    let id3 = client.register_blood(&bank, &BloodType::ONegative, &450u32, &expiration, &None);
    assert_eq!(id3, 3);
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_register_blood_quantity_too_low() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    client.register_blood(
        &bank,
        &BloodType::APositive,
        &50u32, // Too low
        &(current_time + 30 * 86400),
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_register_blood_quantity_too_high() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    client.register_blood(
        &bank,
        &BloodType::APositive,
        &700u32, // Too high
        &(current_time + 30 * 86400),
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_register_blood_expiration_in_past() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    client.register_blood(
        &bank,
        &BloodType::APositive,
        &450u32,
        &(current_time - 100), // In the past
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_register_blood_expiration_too_far_future() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    // 60 days is beyond the 42-day maximum for whole blood
    client.register_blood(
        &bank,
        &BloodType::APositive,
        &450u32,
        &(current_time + 60 * 86400),
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_register_blood_insufficient_shelf_life() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    // Only 12 hours shelf life (less than minimum 1 day)
    client.register_blood(
        &bank,
        &BloodType::APositive,
        &450u32,
        &(current_time + 43200),
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #32)")]
fn test_register_blood_unauthorized_bank() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let unauthorized_bank = Address::generate(&env);
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    client.register_blood(
        &unauthorized_bank,
        &BloodType::APositive,
        &450u32,
        &(current_time + 30 * 86400),
        &None,
    );
}

#[test]
fn test_register_all_blood_types() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let expiration = current_time + (30 * 86400);

    let blood_types = vec![
        &env,
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
        let id = client.register_blood(&bank, &blood_type, &450u32, &expiration, &None);

        assert_eq!(id, (i + 1) as u64);

        let unit = client.get_blood_unit(&id);
        assert_eq!(unit.blood_type, blood_type);
    }
}

#[test]
#[should_panic(expected = "Error(Contract, #21)")]
fn test_get_blood_unit_not_found() {
    let (_env, _admin, client, _contract_id) = create_test_contract();

    client.get_blood_unit(&999);
}

#[test]
fn test_register_blood_edge_case_quantities() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);
    let expiration = current_time + (30 * 86400);

    // Minimum valid quantity
    let id1 = client.register_blood(&bank, &BloodType::APositive, &100u32, &expiration, &None);
    let unit1 = client.get_blood_unit(&id1);
    assert_eq!(unit1.quantity_ml, 100);

    // Maximum valid quantity
    let id2 = client.register_blood(&bank, &BloodType::BPositive, &600u32, &expiration, &None);
    let unit2 = client.get_blood_unit(&id2);
    assert_eq!(unit2.quantity_ml, 600);
}

#[test]
fn test_register_blood_edge_case_expiration() {
    let (env, admin, client, _contract_id) = create_test_contract();

    let bank = admin.clone();
    let current_time = 1000u64;
    env.ledger().set_timestamp(current_time);

    // Minimum shelf life (1 day + 1 second)
    let min_expiration = current_time + 86400 + 1;
    let id1 = client.register_blood(
        &bank,
        &BloodType::APositive,
        &450u32,
        &min_expiration,
        &None,
    );
    let unit1 = client.get_blood_unit(&id1);
    assert_eq!(unit1.expiration_timestamp, min_expiration);

    // Maximum shelf life (42 days)
    let max_expiration = current_time + (42 * 86400);
    let id2 = client.register_blood(
        &bank,
        &BloodType::BPositive,
        &450u32,
        &max_expiration,
        &None,
    );
    let unit2 = client.get_blood_unit(&id2);
    assert_eq!(unit2.expiration_timestamp, max_expiration);
}
