# WebSocket Gateway Documentation

## Overview

The OrdersGateway provides real-time order status updates to connected clients using Socket.IO. It enables hospitals to receive live updates when order statuses change without polling the API.

## Architecture

### Namespace
- **Namespace**: `/orders`
- **URL**: `ws://localhost:3000/orders` (development)

### Room-Based Broadcasting
The gateway uses Socket.IO rooms to ensure hospitals only receive updates for their own orders:
- Room naming pattern: `hospital:{hospitalId}`
- Clients join their hospital's room upon connection
- Updates are broadcast only to clients in the relevant hospital room

## Authentication

### Connection Authentication
The gateway requires authentication via JWT token:

```typescript
// Client-side connection example
const socket = io('http://localhost:3000/orders', {
  auth: {
    token: 'YOUR_JWT_TOKEN'
  }
});
```

Alternative authentication via headers:
```typescript
const socket = io('http://localhost:3000/orders', {
  extraHeaders: {
    authorization: 'Bearer YOUR_JWT_TOKEN'
  }
});
```

### Authentication Middleware
The gateway validates tokens in the `afterInit` lifecycle hook:
- Checks for token in `socket.handshake.auth.token` or `socket.handshake.headers.authorization`
- Rejects connections without valid tokens
- TODO: Implement full JWT validation (signature, expiration, user extraction)

## Events

### Client → Server Events

#### `join:hospital`
Join a hospital-specific room to receive order updates.

**Payload**:
```typescript
{
  hospitalId: string;
}
```

**Example**:
```typescript
socket.emit('join:hospital', { hospitalId: 'HOSP-001' });
```

**Response**:
The server emits a `joined` event confirming the room join:
```typescript
socket.on('joined', (data) => {
  console.log('Joined room:', data.room); // "hospital:HOSP-001"
});
```

### Server → Client Events

#### `order:updated`
Emitted when an order's status changes.

**Payload**:
```typescript
{
  id: string;
  status: OrderStatus;
  rider: RiderInfo | null;
  updatedAt: Date;
  deliveredAt: Date | null;
}
```

**Example**:
```typescript
socket.on('order:updated', (update) => {
  console.log('Order updated:', update);
  // Update UI with new order status
});
```

### Built-in Socket.IO Events

#### `connect`
Emitted when connection is established.

```typescript
socket.on('connect', () => {
  console.log('Connected to WebSocket');
});
```

#### `disconnect`
Emitted when connection is lost.

```typescript
socket.on('disconnect', (reason) => {
  console.log('Disconnected:', reason);
});
```

#### `connect_error`
Emitted when connection fails (e.g., authentication error).

```typescript
socket.on('connect_error', (error) => {
  console.error('Connection error:', error.message);
});
```

## Integration with OrdersService

The gateway is integrated with the `OrdersService.updateStatus` method:

```typescript
async updateStatus(id: string, status: string) {
  // Update order in database
  const updatedOrder = await this.updateOrderInDB(id, status);
  
  // Broadcast update via WebSocket
  this.ordersGateway.emitOrderUpdate(updatedOrder.hospital.id, {
    id: updatedOrder.id,
    status: updatedOrder.status,
    rider: updatedOrder.rider,
    updatedAt: updatedOrder.updatedAt,
    deliveredAt: updatedOrder.deliveredAt,
  });
  
  return updatedOrder;
}
```

## Client Implementation Example

### React/Next.js Client

```typescript
import { useEffect, useState } from 'react';
import { io, Socket } from 'socket.io-client';

function useOrderUpdates(hospitalId: string, authToken: string) {
  const [socket, setSocket] = useState<Socket | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    // Connect to WebSocket
    const newSocket = io('http://localhost:3000/orders', {
      auth: { token: authToken },
      reconnection: true,
      reconnectionDelay: 1000,
      reconnectionAttempts: 5,
    });

    // Connection handlers
    newSocket.on('connect', () => {
      console.log('WebSocket connected');
      setConnected(true);
      
      // Join hospital room
      newSocket.emit('join:hospital', { hospitalId });
    });

    newSocket.on('disconnect', () => {
      console.log('WebSocket disconnected');
      setConnected(false);
    });

    newSocket.on('connect_error', (error) => {
      console.error('WebSocket connection error:', error);
      setConnected(false);
    });

    // Order update handler
    newSocket.on('order:updated', (update) => {
      console.log('Order updated:', update);
      // Update local state with new order data
    });

    setSocket(newSocket);

    // Cleanup on unmount
    return () => {
      newSocket.close();
    };
  }, [hospitalId, authToken]);

  return { socket, connected };
}
```

## Testing

### Unit Tests
The gateway includes comprehensive unit tests covering:
- Connection/disconnection handling
- Room joining
- Order update broadcasting
- Authentication middleware setup

Run tests:
```bash
npm test -- orders.gateway.spec.ts
```

### Manual Testing with Socket.IO Client

Install the Socket.IO client CLI:
```bash
npm install -g socket.io-client
```

Test connection:
```javascript
const io = require('socket.io-client');

const socket = io('http://localhost:3000/orders', {
  auth: { token: 'test-token' }
});

socket.on('connect', () => {
  console.log('Connected!');
  socket.emit('join:hospital', { hospitalId: 'HOSP-001' });
});

socket.on('joined', (data) => {
  console.log('Joined room:', data);
});

socket.on('order:updated', (update) => {
  console.log('Order update received:', update);
});

socket.on('connect_error', (error) => {
  console.error('Connection error:', error.message);
});
```

## Configuration

### CORS Settings
The gateway is configured to accept connections from any origin:
```typescript
@WebSocketGateway({
  cors: {
    origin: '*',
    credentials: true,
  },
  namespace: '/orders',
})
```

**Production**: Update CORS settings to restrict origins:
```typescript
cors: {
  origin: ['https://yourdomain.com'],
  credentials: true,
}
```

### Reconnection Settings
Clients should configure reconnection behavior:
```typescript
const socket = io('http://localhost:3000/orders', {
  reconnection: true,
  reconnectionDelay: 1000,
  reconnectionDelayMax: 5000,
  reconnectionAttempts: 5,
});
```

## Error Handling

### Authentication Errors
If authentication fails, the connection is rejected:
```typescript
socket.on('connect_error', (error) => {
  if (error.message === 'Authentication token required') {
    // Redirect to login or refresh token
  }
});
```

### Connection Failures
Handle connection failures gracefully:
```typescript
socket.on('disconnect', (reason) => {
  if (reason === 'io server disconnect') {
    // Server forcibly disconnected, likely auth issue
    // Don't auto-reconnect, redirect to login
  } else {
    // Network issue, Socket.IO will auto-reconnect
    // Show "Reconnecting..." message to user
  }
});
```

## Performance Considerations

### Room Management
- Rooms are automatically cleaned up when all clients leave
- No manual room cleanup required
- Rooms are memory-efficient for large numbers of hospitals

### Scaling
For horizontal scaling across multiple server instances:
1. Use Redis adapter for Socket.IO
2. Install dependencies:
   ```bash
   npm install @socket.io/redis-adapter redis
   ```
3. Configure adapter in gateway:
   ```typescript
   import { createAdapter } from '@socket.io/redis-adapter';
   import { createClient } from 'redis';
   
   afterInit(server: Server) {
     const pubClient = createClient({ url: 'redis://localhost:6379' });
     const subClient = pubClient.duplicate();
     
     Promise.all([pubClient.connect(), subClient.connect()]).then(() => {
       server.adapter(createAdapter(pubClient, subClient));
     });
   }
   ```

## Security Best Practices

1. **Always validate JWT tokens**: Implement proper JWT validation in production
2. **Use HTTPS/WSS in production**: Encrypt WebSocket connections
3. **Implement rate limiting**: Prevent abuse of WebSocket connections
4. **Validate room access**: Ensure users can only join rooms for their hospital
5. **Sanitize emitted data**: Never emit sensitive data like passwords or full user objects

## Troubleshooting

### Connection Refused
- Check if backend server is running
- Verify WebSocket port is not blocked by firewall
- Ensure CORS settings allow your client origin

### Not Receiving Updates
- Verify client has joined the correct hospital room
- Check server logs for broadcast messages
- Ensure order updates are calling `emitOrderUpdate` method

### Authentication Failures
- Verify JWT token is valid and not expired
- Check token is being sent in correct format
- Review server logs for authentication errors

## Future Enhancements

1. **JWT Validation**: Implement full JWT token validation
2. **User Authorization**: Verify users have access to specific hospitals
3. **Presence Tracking**: Track which users are currently viewing orders
4. **Typing Indicators**: Show when other users are updating orders
5. **Message Acknowledgments**: Confirm clients received updates
6. **Compression**: Enable WebSocket compression for large payloads
