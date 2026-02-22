use soroban_sdk::contracterror;

/// Contract error types for the blood request contract
///
/// Error codes are organized by category:
/// - General errors (0-9): Initialization and general contract errors
/// - Validation errors (10-19): Input validation failures
/// - State errors (20-29): Invalid state or state transitions
/// - Permission errors (30-39): Authorization failures
/// - Request-specific errors (40-49): Blood request specific errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // ========== General errors (0-9) ==========
    /// Contract has already been initialized
    AlreadyInitialized = 0,

    /// Contract has not been initialized
    NotInitialized = 1,

    /// Caller is not authorized to perform this action
    Unauthorized = 2,

    // ========== Validation errors (10-19) ==========
    /// Invalid amount specified
    InvalidAmount = 10,

    /// Invalid address provided
    InvalidAddress = 11,

    /// Invalid input parameter
    InvalidInput = 12,

    /// Invalid blood type specified
    InvalidBloodType = 13,

    /// Invalid status value
    InvalidStatus = 14,

    /// Invalid timestamp (in past, too far in future, etc.)
    InvalidTimestamp = 15,

    /// Invalid quantity (outside acceptable range)
    InvalidQuantity = 16,

    /// Invalid required_by timestamp
    InvalidRequiredBy = 17,

    /// Invalid urgency level
    InvalidUrgency = 18,

    /// Delivery address is empty or invalid
    InvalidDeliveryAddress = 19,

    // ========== State errors (20-29) ==========
    /// Resource already exists (duplicate)
    AlreadyExists = 20,

    /// Resource not found
    NotFound = 21,

    /// Request has expired (required_by timestamp passed)
    RequestExpired = 22,

    /// Request is already in a terminal state
    RequestAlreadyTerminal = 23,

    /// Duplicate request detected
    DuplicateRequest = 24,

    // ========== Permission errors (30-39) ==========
    /// Insufficient balance for operation
    InsufficientBalance = 30,

    /// Insufficient permissions for this action
    InsufficientPermissions = 31,

    /// Hospital is not authorized to make requests
    NotAuthorizedHospital = 32,

    /// Only the request creator can perform this action
    NotRequestCreator = 33,

    // ========== Request-specific errors (40-49) ==========
    /// Request is not in a valid state for this operation
    InvalidRequestState = 40,

    /// Invalid status transition attempted
    InvalidStatusTransition = 41,

    /// Request cannot be cancelled in current state
    CannotCancelRequest = 42,

    /// No blood units available to fulfill request
    NoUnitsAvailable = 43,

    /// Insufficient quantity to fulfill request
    InsufficientQuantity = 44,

    /// Request has already been fulfilled
    AlreadyFulfilled = 45,

    /// Units have already been assigned to this request
    UnitsAlreadyAssigned = 46,

    /// Request is not yet approved
    RequestNotApproved = 47,
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
    RequestExpired = 23,
    DuplicateRequest = 24,

    // Permission errors (30-39)
    InsufficientPermissions = 31,
    NotAuthorizedHospital = 32,
    NotAuthorizedBloodBank = 33,

    // Request-specific errors (40-49)
    RequestNotFound = 40,
    InvalidStatusTransition = 41,
    RequestAlreadyFulfilled = 42,
    InsufficientBloodUnits = 43,
    RequestOverdue = 44,
}
