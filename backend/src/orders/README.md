# Orders Module - Enhanced with Filtering and Pagination

This module has been enhanced to support the Hospital Order History Dashboard feature with comprehensive filtering, sorting, and pagination capabilities.

## Features Implemented

### 1. DTOs (Data Transfer Objects)

#### OrderQueryParamsDto
Located in `dto/order-query-params.dto.ts`

Query parameters for filtering and pagination:
- `hospitalId` (required): Filter orders by hospital
- `startDate` (optional): Start date for date range filter (ISO 8601)
- `endDate` (optional): End date for date range filter (ISO 8601)
- `bloodTypes` (optional): Comma-separated blood types (e.g., "A+,O-,B+")
- `statuses` (optional): Comma-separated statuses (e.g., "pending,confirmed")
- `bloodBank` (optional): Blood bank name search (case-insensitive partial match)
- `sortBy` (optional): Column to sort by (default: "placedAt")
- `sortOrder` (optional): Sort order "asc" or "desc" (default: "desc")
- `page` (optional): Page number (default: 1)
- `pageSize` (optional): Items per page - 25, 50, or 100 (default: 25)

#### OrdersResponseDto
Located in `dto/orders-response.dto.ts`

Response format with pagination metadata:
```typescript
{
  data: Order[],
  pagination: {
    currentPage: number,
    pageSize: number,
    totalCount: number,
    totalPages: number
  }
}
```

### 2. Type Definitions

Located in `types/order.types.ts`

- `BloodType`: Union type for blood types
- `OrderStatus`: Union type for order statuses
- `Order`: Complete order interface
- `BloodBankInfo`, `HospitalInfo`, `RiderInfo`: Supporting interfaces

### 3. Service Methods

#### findAllWithFilters(params: OrderQueryParamsDto)

Comprehensive filtering and pagination method that:

1. **Filters by hospital**: Only returns orders for the specified hospital
2. **Date range filtering**: Filters orders where `placedAt` is within the specified range
3. **Blood type filtering**: Multi-select filter for blood types
4. **Status filtering**: Multi-select filter for order statuses
5. **Blood bank search**: Case-insensitive partial match on blood bank name
6. **Active orders prioritization**: Orders with status "pending", "confirmed", or "in_transit" appear first
7. **Column sorting**: Sorts by specified column in ascending or descending order
8. **Pagination**: Returns paginated results with metadata

### 4. Controller Endpoint

#### GET /orders

Enhanced endpoint that accepts all query parameters defined in `OrderQueryParamsDto`.

Example request:
```
GET /orders?hospitalId=HOSP-001&startDate=2024-01-01&endDate=2024-01-31&bloodTypes=A+,O-&statuses=pending,confirmed&page=1&pageSize=25
```

Example response:
```json
{
  "data": [
    {
      "id": "ORD-001",
      "bloodType": "A+",
      "quantity": 5,
      "bloodBank": {
        "id": "BB-001",
        "name": "Central Blood Bank",
        "location": "Lagos"
      },
      "hospital": {
        "id": "HOSP-001",
        "name": "General Hospital",
        "location": "Ikeja"
      },
      "status": "in_transit",
      "rider": {
        "id": "RIDER-001",
        "name": "John Doe",
        "phone": "+234-XXX-XXXX"
      },
      "placedAt": "2024-01-15T10:30:00Z",
      "deliveredAt": null,
      "confirmedAt": "2024-01-15T10:35:00Z",
      "cancelledAt": null,
      "createdAt": "2024-01-15T10:30:00Z",
      "updatedAt": "2024-01-15T11:00:00Z"
    }
  ],
  "pagination": {
    "currentPage": 1,
    "pageSize": 25,
    "totalCount": 150,
    "totalPages": 6
  }
}
```

### 5. Database Indexes

When a database is configured, run the migration in `migrations/add-orders-indexes.sql` to add the following indexes for optimal performance:

- `idx_orders_hospital_id`: For hospital filtering
- `idx_orders_placed_at`: For date sorting and filtering
- `idx_orders_status`: For status filtering
- `idx_orders_blood_type`: For blood type filtering
- `idx_orders_hospital_placed_at`: Composite index for common query pattern
- `idx_orders_hospital_status`: Composite index for active orders queries
- `idx_orders_blood_bank_name`: For blood bank name search

## Implementation Notes

### Active Orders Prioritization

The sorting logic ensures that active orders (pending, confirmed, in_transit) always appear before completed orders (delivered, cancelled), regardless of the sort column. Within each group (active/completed), orders are sorted according to the specified column and order.

### Page Size Validation

The `pageSize` parameter is validated to only accept values of 25, 50, or 100. Any other value will result in a validation error.

### Case-Insensitive Search

The blood bank name filter performs a case-insensitive partial match, allowing users to search for "central" and match "Central Blood Bank".

### Mock Data Store

Currently, the service uses an in-memory array (`private orders: Order[] = []`) as a mock data store. In production, this should be replaced with actual database queries using an ORM like TypeORM or Prisma.

## Next Steps

1. **Database Integration**: Replace the mock data store with actual database queries
2. **WebSocket Gateway**: Implement real-time order status updates (Task 2)
3. **Frontend Integration**: Build the dashboard UI components (Tasks 3-5)
4. **Testing**: Add unit tests and property-based tests for filtering logic
5. **Authentication**: Add authentication guards to protect the endpoint
6. **Authorization**: Ensure users can only access orders for their own hospital
