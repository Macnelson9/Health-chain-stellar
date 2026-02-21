// WebSocketClient - Manages WebSocket connections for real-time order updates

import { io, Socket } from 'socket.io-client';
import { Order } from '../types/orders';

export class WebSocketClient {
  private socket: Socket | null = null;
  private hospitalId: string;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private baseReconnectDelay = 1000; // 1 second
  private onOrderUpdateCallback?: (order: Partial<Order>) => void;
  private onConnectionChangeCallback?: (connected: boolean) => void;

  constructor(hospitalId: string) {
    this.hospitalId = hospitalId;
  }

  /**
   * Connect to WebSocket server
   * Establishes connection with authentication and joins hospital room
   */
  async connect(authToken?: string): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        // Determine WebSocket URL (use environment variable or default)
        const wsUrl = process.env.NEXT_PUBLIC_WS_URL || 'http://localhost:3000';

        // Create socket connection
        this.socket = io(`${wsUrl}/orders`, {
          auth: authToken ? { token: authToken } : undefined,
          reconnection: true,
          reconnectionDelay: this.baseReconnectDelay,
          reconnectionAttempts: this.maxReconnectAttempts,
          transports: ['websocket', 'polling'],
        });

        // Connection established
        this.socket.on('connect', () => {
          console.log('WebSocket connected');
          this.reconnectAttempts = 0;

          // Join hospital room
          if (this.socket) {
            this.socket.emit('join:hospital', { hospitalId: this.hospitalId });
          }

          // Notify connection change
          if (this.onConnectionChangeCallback) {
            this.onConnectionChangeCallback(true);
          }

          resolve();
        });

        // Connection error
        this.socket.on('connect_error', (error) => {
          console.error('WebSocket connection error:', error);
          this.reconnectAttempts++;

          // Notify connection change
          if (this.onConnectionChangeCallback) {
            this.onConnectionChangeCallback(false);
          }

          // If max attempts reached, reject
          if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            reject(new Error('Failed to connect to WebSocket server'));
          }
        });

        // Disconnection
        this.socket.on('disconnect', (reason) => {
          console.log('WebSocket disconnected:', reason);

          // Notify connection change
          if (this.onConnectionChangeCallback) {
            this.onConnectionChangeCallback(false);
          }

          // Implement exponential backoff for reconnection
          if (reason === 'io server disconnect') {
            // Server disconnected, manually reconnect
            this.reconnectWithBackoff();
          }
        });

        // Listen for order updates
        this.socket.on('order:updated', (update: Partial<Order>) => {
          console.log('Order update received:', update);

          // Call registered callback
          if (this.onOrderUpdateCallback) {
            this.onOrderUpdateCallback(update);
          }
        });

        // Reconnection attempt
        this.socket.on('reconnect_attempt', (attemptNumber) => {
          console.log(`WebSocket reconnection attempt ${attemptNumber}`);
        });

        // Reconnection success
        this.socket.on('reconnect', (attemptNumber) => {
          console.log(`WebSocket reconnected after ${attemptNumber} attempts`);
          this.reconnectAttempts = 0;

          // Rejoin hospital room
          if (this.socket) {
            this.socket.emit('join:hospital', { hospitalId: this.hospitalId });
          }

          // Notify connection change
          if (this.onConnectionChangeCallback) {
            this.onConnectionChangeCallback(true);
          }
        });

        // Reconnection failed
        this.socket.on('reconnect_failed', () => {
          console.error('WebSocket reconnection failed');

          // Notify connection change
          if (this.onConnectionChangeCallback) {
            this.onConnectionChangeCallback(false);
          }
        });
      } catch (error) {
        console.error('Error creating WebSocket connection:', error);
        reject(error);
      }
    });
  }

  /**
   * Reconnect with exponential backoff
   * Implements exponential backoff strategy for reconnection
   */
  private reconnectWithBackoff(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached');
      return;
    }

    const delay = this.baseReconnectDelay * Math.pow(2, this.reconnectAttempts);
    console.log(`Reconnecting in ${delay}ms...`);

    setTimeout(() => {
      this.reconnectAttempts++;
      if (this.socket) {
        this.socket.connect();
      }
    }, delay);
  }

  /**
   * Disconnect from WebSocket server
   * Closes connection and cleans up resources
   */
  disconnect(): void {
    if (this.socket) {
      console.log('Disconnecting WebSocket');
      this.socket.disconnect();
      this.socket = null;
    }

    // Clear callbacks
    this.onOrderUpdateCallback = undefined;
    this.onConnectionChangeCallback = undefined;

    // Reset reconnection attempts
    this.reconnectAttempts = 0;
  }

  /**
   * Register callback for order updates
   * Called when an order status update is received
   */
  onOrderUpdate(callback: (order: Partial<Order>) => void): void {
    this.onOrderUpdateCallback = callback;
  }

  /**
   * Register callback for connection status changes
   * Called when connection is established or lost
   */
  onConnectionChange(callback: (connected: boolean) => void): void {
    this.onConnectionChangeCallback = callback;
  }

  /**
   * Check if socket is connected
   */
  isConnected(): boolean {
    return this.socket?.connected ?? false;
  }
}
