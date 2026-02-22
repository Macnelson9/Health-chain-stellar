use crate::error::ContractError;
use soroban_sdk::{contracttype, Address, Map, String, Symbol, Vec};

/// Blood type enumeration supporting all major blood groups
///
/// Each variant represents a unique combination of ABO and Rh blood typing.
/// This is defined locally for contract independence (Soroban contracts are self-contained).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum BloodType {
    /// Type A positive (A+)
    APositive,
    /// Type A negative (A-)
    ANegative,
    /// Type B positive (B+)
    BPositive,
    /// Type B negative (B-)
    BNegative,
    /// Type AB positive (AB+) - Universal plasma donor
    ABPositive,
    /// Type AB negative (AB-)
    ABNegative,
    /// Type O positive (O+)
    OPositive,
    /// Type O negative (O-) - Universal blood donor
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
/// Determines priority in request processing and fulfillment:
/// - Critical: Life-threatening emergency, immediate fulfillment required
/// - Urgent: High priority, should be fulfilled within hours
/// - Normal: Standard request, can be scheduled for routine delivery
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum UrgencyLevel {
    /// Life-threatening emergency requiring immediate attention
    /// Examples: Active hemorrhage, emergency surgery, trauma
    Critical,
    /// High priority request needing quick response
    /// Examples: Scheduled urgent surgery, declining patient
    Urgent,
    /// Standard priority for routine needs
    /// Examples: Scheduled transfusions, inventory replenishment
    Normal,
}

/// Request status representing the lifecycle of a blood request
///
/// Status transitions follow this flow:
/// Pending -> Approved -> Fulfilled -> InDelivery -> Completed
///        \-> Cancelled (from Pending, Approved, Fulfilled)
///        \-> Expired (from Pending, Approved if required_by passes)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum RequestStatus {
    /// Initial state - request submitted, awaiting approval
    Pending,
    /// Request approved, waiting for blood unit assignment
    Approved,
    /// Blood units have been assigned to fulfill the request
    Fulfilled,
    /// Blood units are in transit to the hospital
    InDelivery,
    /// Request successfully completed, blood delivered
    Completed,
    /// Request was cancelled (by hospital or admin)
    Cancelled,
    /// Request expired before fulfillment (required_by timestamp passed)
    Expired,
}

/// Complete blood request record from a hospital
///
/// Represents a request for blood units with full tracking information
/// from creation through delivery or cancellation.
///
/// # Storage Keys
/// - Primary key: `id` (u64)
/// - Secondary indexes: `hospital_id`, `blood_type`, `status`, `urgency`
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

    /// Hospital address that submitted the request
    pub hospital_id: Address,

    /// Requested blood type (A+, A-, B+, B-, AB+, AB-, O+, O-)
    pub blood_type: BloodType,

    /// Total quantity requested in milliliters (ml)
    /// Hospitals may request multiple units worth (e.g., 900ml = 2 units)
    pub quantity_ml: u32,

    /// Urgency level determining fulfillment priority
    pub urgency: UrgencyLevel,

    /// Current status in the request lifecycle
    pub status: RequestStatus,

    /// Unix timestamp (seconds) when request was created
    pub created_at: u64,

    /// Unix timestamp (seconds) by which blood is required
    /// Request may expire if not fulfilled by this time
    pub required_by: u64,

    /// Unix timestamp (seconds) when request was fulfilled (units assigned)
    /// None if not yet fulfilled
    pub fulfilled_at: Option<u64>,

    /// IDs of blood units assigned to fulfill this request
    /// References blood unit IDs from the inventory contract
    pub assigned_units: Vec<u64>,

    /// Physical delivery address for the blood units
    pub delivery_address: String,

    /// Extensible metadata for additional attributes
    /// Examples: contact_person, phone_number, special_instructions, department
    pub metadata: Map<Symbol, String>,
}

impl UrgencyLevel {
    /// Get the priority weight for sorting (higher = more urgent)
    ///
    /// Used for prioritizing request fulfillment:
    /// - Critical: 3 (highest priority)
    /// - Urgent: 2
    /// - Normal: 1 (lowest priority)
    pub fn priority_weight(&self) -> u32 {
        match self {
            UrgencyLevel::Critical => 3,
            UrgencyLevel::Urgent => 2,
            UrgencyLevel::Normal => 1,
        }
    }

    /// Check if this urgency level is higher than another
    pub fn is_higher_than(&self, other: &UrgencyLevel) -> bool {
        self.priority_weight() > other.priority_weight()
    }
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
    /// - Pending -> Approved, Cancelled, Expired
    /// - Approved -> Fulfilled, Cancelled, Expired
    /// - Fulfilled -> InDelivery, Cancelled
    /// - InDelivery -> Completed, Cancelled
    /// - Completed -> (terminal state)
    /// - Cancelled -> (terminal state)
    /// - Expired -> (terminal state)
    pub fn can_transition_to(&self, new_status: &RequestStatus) -> bool {
        use RequestStatus::*;

        match (self, new_status) {
            // Pending can go to Approved, Cancelled, or Expired
            (Pending, Approved) => true,
            (Pending, Cancelled) => true,
            (Pending, Expired) => true,

            // Approved can go to Fulfilled, Cancelled, or Expired
            (Approved, Fulfilled) => true,
            (Approved, Cancelled) => true,
            (Approved, Expired) => true,

            // Fulfilled can go to InDelivery or Cancelled
            (Fulfilled, InDelivery) => true,
            (Fulfilled, Cancelled) => true,

            // InDelivery can go to Completed or Cancelled
            (InDelivery, Completed) => true,
            (InDelivery, Cancelled) => true,

            // Terminal states cannot transition
            (Completed, _) => false,
            (Cancelled, _) => false,
            (Expired, _) => false,

            // No other transitions allowed
            _ => false,
        }
    }

    /// Check if this status is a terminal state (no further transitions possible)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            RequestStatus::Completed | RequestStatus::Cancelled | RequestStatus::Expired
        )
    }

    /// Check if request is still active (can be modified or fulfilled)
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            RequestStatus::Pending
                | RequestStatus::Approved
                | RequestStatus::Fulfilled
                | RequestStatus::InDelivery
        )
    }

    /// Check if request can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(
            self,
            RequestStatus::Pending
                | RequestStatus::Approved
                | RequestStatus::Fulfilled
                | RequestStatus::InDelivery
        )
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
    /// Validate that the blood request data is consistent and valid
    ///
    /// Checks:
    /// - Quantity is within acceptable range (100-10000ml)
    /// - required_by is after created_at
    /// - Timestamps are reasonable
    pub fn validate(&self, current_time: u64) -> Result<(), ContractError> {
        // Validate quantity (typical range: 100-10000ml for hospital requests)
        if self.quantity_ml < 100 || self.quantity_ml > 10000 {
            return Err(ContractError::InvalidQuantity);
        }

        // Validate timestamps - required_by must be after created_at
        if self.required_by <= self.created_at {
            return Err(ContractError::InvalidTimestamp);
        }

        // created_at shouldn't be too far in the future (allow 1 hour for clock skew)
        if self.created_at > current_time + 3600 {
            return Err(ContractError::InvalidTimestamp);
        }

        // Validate fulfilled_at if present
        if let Some(fulfilled) = self.fulfilled_at {
            if fulfilled < self.created_at {
                return Err(ContractError::InvalidTimestamp);
            }
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

    /// Check if request has expired based on required_by timestamp
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.required_by
    }

    /// Calculate time remaining until required_by in seconds
    /// Returns negative value if already past required_by
    pub fn time_remaining(&self, current_time: u64) -> i64 {
        (self.required_by as i64) - (current_time as i64)
    }

    /// Check if the request has been fulfilled (has assigned units)
    pub fn has_assigned_units(&self) -> bool {
        self.assigned_units.len() > 0
    }
}

/// Storage key types for efficient querying of blood requests
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    /// Individual blood request by ID
    BloodRequest(u64),

    /// Counter for generating new request IDs
    RequestCounter,

    /// Index: Hospital ID -> Vec<u64> (request IDs)
    HospitalIndex(Address),

    /// Index: Blood type -> Vec<u64> (request IDs)
    BloodTypeIndex(BloodType),

    /// Index: Status -> Vec<u64> (request IDs)
    StatusIndex(RequestStatus),

    /// Index: Urgency level -> Vec<u64> (request IDs)
    UrgencyIndex(UrgencyLevel),

    /// Admin address
    Admin,

    /// Authorized hospitals set
    AuthorizedHospital(Address),
}

/// Event emitted when a new blood request is created
#[contracttype]
#[derive(Clone, Debug)]
pub struct RequestCreatedEvent {
    /// Unique ID of the created request
    pub request_id: u64,

    /// Hospital that created the request
    pub hospital_id: Address,

    /// Requested blood type
    pub blood_type: BloodType,

    /// Quantity requested in milliliters
    pub quantity_ml: u32,

    /// Urgency level
    pub urgency: UrgencyLevel,

    /// When blood is required by
    pub required_by: u64,

    /// When the request was created
    pub created_at: u64,
}

/// Event emitted when a request status changes
#[contracttype]
#[derive(Clone, Debug)]
pub struct RequestStatusChangedEvent {
    /// Request ID
    pub request_id: u64,

    /// Previous status
    pub old_status: RequestStatus,

    /// New status
    pub new_status: RequestStatus,

    /// Timestamp of the status change
    pub changed_at: u64,
}

/// Event emitted when blood units are assigned to a request
#[contracttype]
#[derive(Clone, Debug)]
pub struct UnitsAssignedEvent {
    /// Request ID
    pub request_id: u64,

    /// IDs of assigned blood units
    pub unit_ids: Vec<u64>,

    /// Total quantity assigned in milliliters
    pub total_quantity_ml: u32,

    /// Timestamp when units were assigned
    pub assigned_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_urgency_level_priority() {
        assert_eq!(UrgencyLevel::Critical.priority_weight(), 3);
        assert_eq!(UrgencyLevel::Urgent.priority_weight(), 2);
        assert_eq!(UrgencyLevel::Normal.priority_weight(), 1);

        assert!(UrgencyLevel::Critical.is_higher_than(&UrgencyLevel::Urgent));
        assert!(UrgencyLevel::Urgent.is_higher_than(&UrgencyLevel::Normal));
        assert!(!UrgencyLevel::Normal.is_higher_than(&UrgencyLevel::Critical));
    }

    #[test]
    fn test_request_status_transitions_from_pending() {
        use RequestStatus::*;

        assert!(Pending.can_transition_to(&Approved));
        assert!(Pending.can_transition_to(&Cancelled));
        assert!(Pending.can_transition_to(&Expired));
        assert!(!Pending.can_transition_to(&Fulfilled));
        assert!(!Pending.can_transition_to(&InDelivery));
        assert!(!Pending.can_transition_to(&Completed));
    }

    #[test]
    fn test_request_status_transitions_from_approved() {
        use RequestStatus::*;

        assert!(Approved.can_transition_to(&Fulfilled));
        assert!(Approved.can_transition_to(&Cancelled));
        assert!(Approved.can_transition_to(&Expired));
        assert!(!Approved.can_transition_to(&Pending));
        assert!(!Approved.can_transition_to(&InDelivery));
        assert!(!Approved.can_transition_to(&Completed));
    }

    #[test]
    fn test_request_status_transitions_from_fulfilled() {
        use RequestStatus::*;

        assert!(Fulfilled.can_transition_to(&InDelivery));
        assert!(Fulfilled.can_transition_to(&Cancelled));
        assert!(!Fulfilled.can_transition_to(&Pending));
        assert!(!Fulfilled.can_transition_to(&Approved));
        assert!(!Fulfilled.can_transition_to(&Completed));
        assert!(!Fulfilled.can_transition_to(&Expired));
    }

    #[test]
    fn test_request_status_transitions_from_in_delivery() {
        use RequestStatus::*;

        assert!(InDelivery.can_transition_to(&Completed));
        assert!(InDelivery.can_transition_to(&Cancelled));
        assert!(!InDelivery.can_transition_to(&Pending));
        assert!(!InDelivery.can_transition_to(&Approved));
        assert!(!InDelivery.can_transition_to(&Fulfilled));
        assert!(!InDelivery.can_transition_to(&Expired));
    }

    #[test]
    fn test_request_status_terminal_states() {
        assert!(RequestStatus::Completed.is_terminal());
        assert!(RequestStatus::Cancelled.is_terminal());
        assert!(RequestStatus::Expired.is_terminal());
        assert!(!RequestStatus::Pending.is_terminal());
        assert!(!RequestStatus::Approved.is_terminal());
        assert!(!RequestStatus::Fulfilled.is_terminal());
        assert!(!RequestStatus::InDelivery.is_terminal());
    }

    #[test]
    fn test_request_status_terminal_cannot_transition() {
        use RequestStatus::*;

        // Completed cannot transition to anything
        assert!(!Completed.can_transition_to(&Pending));
        assert!(!Completed.can_transition_to(&Cancelled));

        // Cancelled cannot transition to anything
        assert!(!Cancelled.can_transition_to(&Pending));
        assert!(!Cancelled.can_transition_to(&Approved));

        // Expired cannot transition to anything
        assert!(!Expired.can_transition_to(&Pending));
        assert!(!Expired.can_transition_to(&Approved));
    }

    #[test]
    fn test_request_status_is_active() {
        assert!(RequestStatus::Pending.is_active());
        assert!(RequestStatus::Approved.is_active());
        assert!(RequestStatus::Fulfilled.is_active());
        assert!(RequestStatus::InDelivery.is_active());
        assert!(!RequestStatus::Completed.is_active());
        assert!(!RequestStatus::Cancelled.is_active());
        assert!(!RequestStatus::Expired.is_active());
    }

    #[test]
    fn test_request_status_can_cancel() {
        assert!(RequestStatus::Pending.can_cancel());
        assert!(RequestStatus::Approved.can_cancel());
        assert!(RequestStatus::Fulfilled.can_cancel());
        assert!(RequestStatus::InDelivery.can_cancel());
        assert!(!RequestStatus::Completed.can_cancel());
        assert!(!RequestStatus::Cancelled.can_cancel());
        assert!(!RequestStatus::Expired.can_cancel());
    }

    #[test]
    fn test_blood_request_validation_valid() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let current_time = 1000u64;

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 900, // 2 units worth
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by: current_time + (7 * 24 * 60 * 60), // 7 days
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        assert!(request.validate(current_time).is_ok());
    }

    #[test]
    fn test_blood_request_validation_invalid_quantity_too_low() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let current_time = 1000u64;

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 50, // Too low
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by: current_time + (7 * 24 * 60 * 60),
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        assert_eq!(
            request.validate(current_time),
            Err(ContractError::InvalidQuantity)
        );
    }

    #[test]
    fn test_blood_request_validation_invalid_quantity_too_high() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let current_time = 1000u64;

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 20000, // Too high
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by: current_time + (7 * 24 * 60 * 60),
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        assert_eq!(
            request.validate(current_time),
            Err(ContractError::InvalidQuantity)
        );
    }

    #[test]
    fn test_blood_request_validation_required_by_before_created() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let current_time = 1000u64;

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 450,
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by: current_time - 100, // Before created_at
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        assert_eq!(
            request.validate(current_time),
            Err(ContractError::InvalidTimestamp)
        );
    }

    #[test]
    fn test_blood_request_is_expired() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let created_time = 1000u64;
        let required_by = created_time + 3600; // 1 hour

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 450,
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: created_time,
            required_by,
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        // Not expired before required_by
        assert!(!request.is_expired(required_by - 1));

        // Expired at required_by
        assert!(request.is_expired(required_by));

        // Expired after required_by
        assert!(request.is_expired(required_by + 100));
    }

    #[test]
    fn test_blood_request_time_remaining() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let created_time = 1000u64;
        let required_by = created_time + 3600; // 1 hour

        let request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 450,
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: created_time,
            required_by,
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        // 30 minutes before required_by
        assert_eq!(request.time_remaining(created_time + 1800), 1800);

        // At required_by
        assert_eq!(request.time_remaining(required_by), 0);

        // 10 minutes past required_by
        assert_eq!(request.time_remaining(required_by + 600), -600);
    }

    #[test]
    fn test_blood_request_has_assigned_units() {
        let env = Env::default();
        let hospital = Address::generate(&env);
        let current_time = 1000u64;

        let mut request = BloodRequest {
            id: 1,
            hospital_id: hospital,
            blood_type: BloodType::APositive,
            quantity_ml: 450,
            urgency: UrgencyLevel::Normal,
            status: RequestStatus::Pending,
            created_at: current_time,
            required_by: current_time + 3600,
            fulfilled_at: None,
            assigned_units: Vec::new(&env),
            delivery_address: String::from_str(&env, "123 Hospital St"),
            metadata: Map::new(&env),
        };

        assert!(!request.has_assigned_units());

        request.assigned_units.push_back(1);
        assert!(request.has_assigned_units());
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
