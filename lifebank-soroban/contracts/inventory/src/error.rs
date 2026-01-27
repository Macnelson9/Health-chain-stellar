use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // General errors (0-9)
    AlreadyInitialized = 0,
    NotInitialized = 1,
    Unauthorized = 2,

    // Validation errors (10-19)
    InvalidAmount = 10,
    InvalidAddress = 11,
    InvalidInput = 12,
    InvalidBloodType = 13,
    InvalidStatus = 14,
    InvalidTimestamp = 15,
    InvalidQuantity = 16,
    InvalidExpiration = 17,

    // State errors (20-29)
    AlreadyExists = 20,
    NotFound = 21,
    Expired = 22,
    BloodUnitExpired = 23,
    DuplicateBloodUnit = 24,

    // Permission errors (30-39)
    InsufficientBalance = 30,
    InsufficientPermissions = 31,
    NotAuthorizedBloodBank = 32,

    // Blood-specific errors (40-49)
    BloodUnitNotAvailable = 40,
    InvalidStatusTransition = 41,
}
