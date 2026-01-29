use soroban_sdk::{contracttype, Address, Vec};

/// Blood type enumeration supporting all major blood groups
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum BloodType {
    APositive,
    ANegative,
    BPositive,
    BNegative,
    ABPositive,
    ABNegative,
    OPositive,
    ONegative,
}

/// Urgency level for blood requests
///
/// Determines priority and fulfillment timeline:
/// - Critical: Life-threatening, immediate fulfillment required
/// - Urgent: High priority, fulfillment within hours
/// - Normal: Standard priority, fulfillment within days
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy, PartialOrd, Ord)]
pub enum UrgencyLevel {
    /// Life-threatening situation, immediate fulfillment required
    Critical,
    /// High priority, fulfillment within hours
    Urgent,
    /// Standard priority, fulfillment within days
    Normal,
}

/// Request status representing its lifecycle
///
/// Status transitions:
/// Pending -> Approved -> Fulfilled -> Completed
///        \-> Rejected
///        \-> Cancelled
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum RequestStatus {
    /// Initial state, awaiting approval
    Pending,
    /// Approved by blood bank, awaiting fulfillment
    Approved,
    /// Blood units assigned and being prepared
    Fulfilled,
    /// Request completed successfully
    Completed,
    /// Request rejected by blood bank
    Rejected,
    /// Request cancelled by hospital
    Cancelled,
}

/// Request metadata containing additional context
#[contracttype]
#[derive(Clone, Debug)]
pub struct RequestMetadata {
    /// Patient name or identifier
    pub patient_id: Address,
    /// Medical procedure or reason for request
    pub procedure: soroban_sdk::String,
    /// Special notes or requirements
    pub notes: soroban_sdk::String,
}

/// Complete blood request record
///
/// Represents a hospital's request for blood units with full tracking
/// from creation through fulfillment or cancellation.
#[contracttype]
#[derive(Clone, Debug)]
pub struct BloodRequest {
    /// Unique identifier for this request
    pub id: u64,

    /// Hospital address requesting blood
    pub hospital_id: Address,

    /// Blood type requested
    pub blood_type: BloodType,

    /// Quantity requested in milliliters
    pub quantity_ml: u32,

    /// Urgency level of the request
    pub urgency: UrgencyLevel,

    /// Current status of the request
    pub status: RequestStatus,

    /// Unix timestamp when request was created
    pub created_at: u64,

    /// Unix timestamp when blood is required by
    pub required_by: u64,

    /// Unix timestamp when request was fulfilled (if applicable)
    pub fulfilled_at: Option<u64>,

    /// Vector of blood unit IDs assigned to this request
    pub assigned_units: Vec<u64>,

    /// Delivery address for the blood units
    pub delivery_address: soroban_sdk::String,

    /// Request metadata (patient info, procedure, notes)
    pub metadata: RequestMetadata,
}

impl RequestStatus {
    /// Check if transition from current status to new status is valid
    ///
    /// Valid transitions:
    /// - Pending -> Approved, Rejected, Cancelled
    /// - Approved -> Fulfilled, Cancelled
    /// - Fulfilled -> Completed
    /// - Rejected, Completed, Cancelled -> (terminal states)
    pub fn can_transition_to(&self, new_status: &RequestStatus) -> bool {
        use RequestStatus::*;

        match (self, new_status) {
            // From Pending
            (Pending, Approved) => true,
            (Pending, Rejected) => true,
            (Pending, Cancelled) => true,

            // From Approved
            (Approved, Fulfilled) => true,
            (Approved, Cancelled) => true,

            // From Fulfilled
            (Fulfilled, Completed) => true,

            // Terminal states - no transitions allowed
            (Rejected, _) => false,
            (Completed, _) => false,
            (Cancelled, _) => false,

            // All other combinations invalid
            _ => false,
        }
    }

    /// Check if this status is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, RequestStatus::Rejected | RequestStatus::Completed | RequestStatus::Cancelled)
    }
}

impl UrgencyLevel {
    /// Get the maximum time allowed for fulfillment in seconds
    ///
    /// - Critical: 1 hour (3600 seconds)
    /// - Urgent: 6 hours (21600 seconds)
    /// - Normal: 24 hours (86400 seconds)
    pub fn max_fulfillment_time(&self) -> u64 {
        match self {
            UrgencyLevel::Critical => 3600,      // 1 hour
            UrgencyLevel::Urgent => 21600,       // 6 hours
            UrgencyLevel::Normal => 86400,       // 24 hours
        }
    }
}

impl BloodRequest {
    /// Validate the blood request
    ///
    /// Checks:
    /// - Quantity is within acceptable range (50-5000ml)
    /// - Required_by is in the future
    /// - Required_by is reasonable relative to created_at
    /// - Delivery address is not empty
    pub fn validate(&self, current_time: u64) -> Result<(), crate::error::ContractError> {
        use crate::error::ContractError;

        // Validate quantity (50-5000ml for hospital requests)
        if self.quantity_ml < 50 || self.quantity_ml > 5000 {
            return Err(ContractError::InvalidQuantity);
        }

        // Required_by must be in the future
        if self.required_by <= current_time {
            return Err(ContractError::InvalidTimestamp);
        }

        // Required_by should be reasonable (not more than 30 days in future)
        let max_future = current_time + (30 * 86400);
        if self.required_by > max_future {
            return Err(ContractError::InvalidTimestamp);
        }

        // Created_at should be before required_by
        if self.created_at >= self.required_by {
            return Err(ContractError::InvalidTimestamp);
        }

        // Delivery address should not be empty
        if self.delivery_address.len() == 0 {
            return Err(ContractError::InvalidInput);
        }

        Ok(())
    }

    /// Check if request has exceeded its required_by deadline
    pub fn is_overdue(&self, current_time: u64) -> bool {
        current_time > self.required_by
    }

    /// Get time remaining until required_by deadline in seconds
    /// Returns negative value if overdue
    pub fn time_remaining(&self, current_time: u64) -> i64 {
        self.required_by as i64 - current_time as i64
    }

    /// Check if request can be fulfilled based on urgency and time
    pub fn can_fulfill(&self, current_time: u64) -> bool {
        !self.is_overdue(current_time) && self.status == RequestStatus::Approved
    }
}

/// Storage key enumeration for efficient indexing
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    /// Primary key for blood requests
    BloodRequest(u64),
    /// Counter for generating request IDs
    RequestCounter,
    /// Index by hospital ID
    HospitalIndex(Address),
    /// Index by blood type
    BloodTypeIndex(BloodType),
    /// Index by status
    StatusIndex(RequestStatus),
    /// Index by urgency level
    UrgencyIndex(UrgencyLevel),
    /// Admin address
    Admin,
}
