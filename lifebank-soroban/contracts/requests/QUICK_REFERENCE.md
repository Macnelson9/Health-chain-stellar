# Blood Request Contract - Quick Reference

## Data Structures

### BloodRequest
Main request record with complete tracking information.

**Key Fields:**
- `id: u64` - Unique identifier
- `hospital_id: Address` - Requesting hospital
- `blood_type: BloodType` - Type of blood (A+, A-, B+, B-, AB+, AB-, O+, O-)
- `quantity_ml: u32` - Amount in milliliters (50-5000ml)
- `urgency: UrgencyLevel` - Priority (Critical, Urgent, Normal)
- `status: RequestStatus` - Current state
- `created_at: u64` - Creation timestamp
- `required_by: u64` - Deadline timestamp
- `fulfilled_at: Option<u64>` - Fulfillment timestamp
- `assigned_units: Vec<u64>` - Blood unit IDs
- `delivery_address: String` - Delivery location
- `metadata: RequestMetadata` - Patient info, procedure, notes

### RequestMetadata
Additional context for the request.

**Fields:**
- `patient_id: Address` - Patient identifier
- `procedure: String` - Medical procedure
- `notes: String` - Special requirements

## Enums

### UrgencyLevel
```
Critical  → 1 hour max fulfillment (3600 seconds)
Urgent    → 6 hours max fulfillment (21600 seconds)
Normal    → 24 hours max fulfillment (86400 seconds)
```

### RequestStatus
```
Pending   → Initial state
Approved  → Approved by blood bank
Fulfilled → Blood units assigned
Completed → Successfully completed
Rejected  → Rejected by blood bank
Cancelled → Cancelled by hospital
```

**Valid Transitions:**
- Pending → Approved, Rejected, Cancelled
- Approved → Fulfilled, Cancelled
- Fulfilled → Completed
- Terminal states: Rejected, Completed, Cancelled (no further transitions)

### BloodType
```
APositive, ANegative
BPositive, BNegative
ABPositive, ABNegative
OPositive, ONegative
```

## Contract Functions

### initialize(admin: Address)
Initialize contract with admin address.
- **Auth Required**: Yes (admin)
- **Returns**: Result<(), ContractError>
- **Errors**: AlreadyInitialized

### create_request(...)
Create new blood request.
```rust
create_request(
    hospital_id: Address,
    blood_type: BloodType,
    quantity_ml: u32,
    urgency: UrgencyLevel,
    required_by: u64,
    delivery_address: String,
    patient_id: Address,
    procedure: String,
    notes: String,
) -> Result<u64, ContractError>
```
- **Auth Required**: Yes (hospital)
- **Returns**: Request ID
- **Validation**: Quantity (50-5000ml), timestamps, delivery address
- **Events**: RequestCreatedEvent

### update_request_status(request_id: u64, new_status: RequestStatus)
Update request status with state machine validation.
- **Auth Required**: Yes (admin)
- **Returns**: Result<(), ContractError>
- **Validation**: Valid status transition
- **Side Effects**: Sets fulfilled_at if transitioning to Fulfilled
- **Events**: RequestStatusChangedEvent

### assign_blood_units(request_id: u64, unit_ids: Vec<u64>)
Assign blood units to request.
- **Auth Required**: Yes (admin)
- **Returns**: Result<(), ContractError>
- **Events**: UnitsAssignedEvent

### get_request(request_id: u64)
Retrieve request by ID.
- **Auth Required**: No
- **Returns**: Result<BloodRequest, ContractError>
- **Errors**: RequestNotFound

## Validation Rules

### Quantity
- Minimum: 50ml
- Maximum: 5000ml
- Error: InvalidQuantity

### Timestamps
- `required_by` must be in future
- `required_by` max 30 days in future
- `created_at` must be before `required_by`
- Error: InvalidTimestamp

### Delivery Address
- Cannot be empty
- Error: InvalidInput

### Authorization
- Hospital must be authorized (admin)
- Error: NotAuthorizedHospital

## Error Codes

| Code | Error | Meaning |
|------|-------|---------|
| 0 | AlreadyInitialized | Contract already initialized |
| 1 | NotInitialized | Contract not initialized |
| 2 | Unauthorized | Caller not authorized |
| 12 | InvalidInput | Invalid input (e.g., empty address) |
| 15 | InvalidTimestamp | Timestamp validation failed |
| 16 | InvalidQuantity | Quantity out of range |
| 32 | NotAuthorizedHospital | Hospital not authorized |
| 40 | RequestNotFound | Request ID not found |
| 41 | InvalidStatusTransition | Status transition not allowed |
| 44 | RequestOverdue | Request past deadline |

## BloodRequest Methods

### validate(current_time: u64) -> Result<(), ContractError>
Validate all request fields.

### is_overdue(current_time: u64) -> bool
Check if request exceeded deadline.

### time_remaining(current_time: u64) -> i64
Get seconds until deadline (negative if overdue).

### can_fulfill(current_time: u64) -> bool
Check if request can be fulfilled (not overdue + Approved status).

## RequestStatus Methods

### can_transition_to(new_status: &RequestStatus) -> bool
Check if status transition is valid.

### is_terminal() -> bool
Check if status is terminal (no further transitions allowed).

## UrgencyLevel Methods

### max_fulfillment_time() -> u64
Get maximum fulfillment time in seconds.

## Events

### RequestCreatedEvent
Emitted when request is created.
- request_id, hospital_id, blood_type, quantity_ml
- urgency, required_by, created_at

### RequestStatusChangedEvent
Emitted when status changes.
- request_id, old_status, new_status, changed_at

### UnitsAssignedEvent
Emitted when blood units are assigned.
- request_id, assigned_units, assigned_at

## Common Workflows

### Create and Approve Request
```
1. create_request(...) → request_id
2. update_request_status(request_id, Approved)
3. assign_blood_units(request_id, [unit_ids])
4. update_request_status(request_id, Fulfilled)
5. update_request_status(request_id, Completed)
```

### Reject Request
```
1. create_request(...) → request_id
2. update_request_status(request_id, Rejected)
```

### Cancel Request
```
1. create_request(...) → request_id
2. update_request_status(request_id, Cancelled)
```

## Storage

### Primary Keys
- `BloodRequest(u64)` - Request by ID
- `RequestCounter` - Counter for ID generation
- `Admin` - Admin address

### Indexes
- `HospitalIndex(Address)` - Requests by hospital
- `BloodTypeIndex(BloodType)` - Requests by blood type
- `StatusIndex(RequestStatus)` - Requests by status
- `UrgencyIndex(UrgencyLevel)` - Requests by urgency

## Testing

Run all tests:
```bash
cargo test --lib
```

Run specific test:
```bash
cargo test --lib test_create_request_as_admin_success
```

All 24 tests passing ✅
