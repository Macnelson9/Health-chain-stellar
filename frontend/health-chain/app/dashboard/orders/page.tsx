'use client';

// OrdersPage - Main page component for Hospital Order History Dashboard

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { Order, OrderFilters, SortConfig, PaginationConfig, OrdersResponse } from '@/lib/types/orders';
import { URLStateManager } from '@/lib/utils/url-state-manager';
import { WebSocketClient } from '@/lib/utils/websocket-client';
import { CSVExporter } from '@/lib/utils/csv-exporter';
import { FilterPanel } from '@/components/orders/FilterPanel';
import { OrderTable } from '@/components/orders/OrderTable';
import { PaginationController } from '@/components/orders/PaginationController';

/**
 * OrdersPage Component
 * Main dashboard page for viewing and managing hospital orders
 */
export default function OrdersPage() {
  // State management
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<OrderFilters>({
    startDate: null,
    endDate: null,
    bloodTypes: [],
    statuses: [],
    bloodBank: '',
  });
  const [sort, setSort] = useState<SortConfig>({
    column: 'placedAt',
    order: 'desc',
  });
  const [pagination, setPagination] = useState<PaginationConfig>({
    page: 1,
    pageSize: 25,
  });
  const [totalCount, setTotalCount] = useState(0);
  const [wsConnected, setWsConnected] = useState(false);

  // WebSocket client ref
  const wsClientRef = useRef<WebSocketClient | null>(null);

  // Initialize state from URL on mount
  useEffect(() => {
    const urlState = URLStateManager.readFromURL();
    setFilters(urlState.filters);
    setSort(urlState.sort);
    setPagination({
      page: urlState.pagination.page,
      pageSize: urlState.pagination.pageSize,
    });
  }, []);

  // Fetch orders from API
  const fetchOrders = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      // Build query parameters
      const params = new URLSearchParams();
      
      // TODO: Replace with actual hospital ID from auth context
      params.set('hospitalId', 'HOSP-001');

      // Date filters
      if (filters.startDate) {
        params.set('startDate', filters.startDate.toISOString().split('T')[0]);
      }
      if (filters.endDate) {
        params.set('endDate', filters.endDate.toISOString().split('T')[0]);
      }

      // Blood types filter
      if (filters.bloodTypes.length > 0) {
        params.set('bloodTypes', filters.bloodTypes.join(','));
      }

      // Statuses filter
      if (filters.statuses.length > 0) {
        params.set('statuses', filters.statuses.join(','));
      }

      // Blood bank filter
      if (filters.bloodBank) {
        params.set('bloodBank', filters.bloodBank);
      }

      // Sort parameters
      params.set('sortBy', sort.column);
      params.set('sortOrder', sort.order);

      // Pagination parameters
      params.set('page', pagination.page.toString());
      params.set('pageSize', pagination.pageSize.toString());

      // Make API request
      const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';
      const apiPrefix = process.env.NEXT_PUBLIC_API_PREFIX || 'api/v1';
      const fullUrl = `${apiUrl}/${apiPrefix}/orders?${params.toString()}`;
      
      console.log('Fetching orders from:', fullUrl);
      console.log('Environment variables:', { apiUrl, apiPrefix });
      
      const response = await fetch(fullUrl);

      if (!response.ok) {
        if (response.status === 401) {
          throw new Error('Session expired. Please log in again.');
        }
        if (response.status === 403) {
          throw new Error('You do not have access to this hospital.');
        }
        if (response.status === 404) {
          throw new Error('Orders API endpoint not found. Please ensure the backend server is running on http://localhost:3000');
        }
        
        // Try to get error details from response
        let errorMessage = 'Failed to fetch orders. Please try again.';
        try {
          const errorData = await response.json();
          if (errorData.message) {
            errorMessage = `API Error: ${errorData.message}`;
          }
        } catch {
          // If we can't parse the error, use the default message
          errorMessage = `API Error (${response.status}): ${response.statusText}. Please ensure the backend server is running.`;
        }
        
        throw new Error(errorMessage);
      }

      const data: OrdersResponse = await response.json();
      
      // Convert date strings to Date objects
      const ordersWithDates = data.data.map((order) => ({
        ...order,
        placedAt: new Date(order.placedAt),
        deliveredAt: order.deliveredAt ? new Date(order.deliveredAt) : null,
        confirmedAt: order.confirmedAt ? new Date(order.confirmedAt) : null,
        cancelledAt: order.cancelledAt ? new Date(order.cancelledAt) : null,
        createdAt: new Date(order.createdAt),
        updatedAt: new Date(order.updatedAt),
      }));

      setOrders(ordersWithDates);
      setTotalCount(data.pagination.totalCount);
    } catch (err) {
      console.error('Error fetching orders:', err);
      console.error('Error details:', {
        message: err instanceof Error ? err.message : 'Unknown error',
        stack: err instanceof Error ? err.stack : undefined,
        type: err instanceof TypeError ? 'Network/CORS error' : 'Other error'
      });
      setError(err instanceof Error ? err.message : 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  }, [filters, sort, pagination]);

  // Fetch orders when dependencies change
  useEffect(() => {
    fetchOrders();
  }, [fetchOrders]);

  // Set up WebSocket connection
  useEffect(() => {
    // TODO: Replace with actual hospital ID from auth context
    const hospitalId = 'HOSP-001';
    
    const wsClient = new WebSocketClient(hospitalId);
    wsClientRef.current = wsClient;

    // Connect to WebSocket
    wsClient
      .connect()
      .then(() => {
        console.log('WebSocket connected successfully');
      })
      .catch((err) => {
        console.error('WebSocket connection failed:', err);
      });

    // Register connection status callback
    wsClient.onConnectionChange((connected) => {
      setWsConnected(connected);
    });

    // Register order update callback
    wsClient.onOrderUpdate((updatedOrder) => {
      setOrders((prevOrders) => {
        return prevOrders.map((order) =>
          order.id === updatedOrder.id
            ? {
                ...order,
                ...updatedOrder,
                updatedAt: updatedOrder.updatedAt
                  ? new Date(updatedOrder.updatedAt)
                  : order.updatedAt,
                deliveredAt: updatedOrder.deliveredAt
                  ? new Date(updatedOrder.deliveredAt)
                  : order.deliveredAt,
              }
            : order
        );
      });
    });

    // Cleanup on unmount
    return () => {
      wsClient.disconnect();
    };
  }, []);

  // Handle filter changes
  const handleFiltersChange = useCallback(
    (newFilters: OrderFilters) => {
      setFilters(newFilters);
      setPagination((prev) => ({ ...prev, page: 1 })); // Reset to page 1
      URLStateManager.updateURL(newFilters, sort, { ...pagination, page: 1 });
    },
    [sort, pagination]
  );

  // Handle clear filters
  const handleClearFilters = useCallback(() => {
    const defaultFilters: OrderFilters = {
      startDate: null,
      endDate: null,
      bloodTypes: [],
      statuses: [],
      bloodBank: '',
    };
    setFilters(defaultFilters);
    setPagination((prev) => ({ ...prev, page: 1 }));
    URLStateManager.updateURL(defaultFilters, sort, { ...pagination, page: 1 });
  }, [sort, pagination]);

  // Handle sort changes
  const handleSortChange = useCallback(
    (column: string) => {
      const newSort: SortConfig = {
        column,
        order: sort.column === column && sort.order === 'asc' ? 'desc' : 'asc',
      };
      setSort(newSort);
      URLStateManager.updateURL(filters, newSort, pagination);
    },
    [filters, sort, pagination]
  );

  // Handle page change
  const handlePageChange = useCallback(
    (page: number) => {
      setPagination((prev) => ({ ...prev, page }));
      URLStateManager.updateURL(filters, sort, { ...pagination, page });
    },
    [filters, sort, pagination]
  );

  // Handle page size change
  const handlePageSizeChange = useCallback(
    (pageSize: number) => {
      setPagination({ page: 1, pageSize: pageSize as 25 | 50 | 100 });
      URLStateManager.updateURL(filters, sort, { page: 1, pageSize: pageSize as 25 | 50 | 100 });
    },
    [filters, sort]
  );

  // Handle CSV export
  const handleExport = useCallback(() => {
    CSVExporter.export(orders);
  }, [orders]);

  // Handle retry
  const handleRetry = useCallback(() => {
    fetchOrders();
  }, [fetchOrders]);

  return (
    <div className="max-w-[1600px] mx-auto">
      {/* Page Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900">Order History</h1>
        <p className="text-gray-600 mt-2">
          View and manage your hospital&apos;s blood order history
        </p>
      </div>

      {/* WebSocket Connection Warning */}
      {!wsConnected && !loading && (
        <div className="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg flex items-center gap-2">
          <AlertCircle className="w-5 h-5 text-yellow-600 flex-shrink-0" />
          <span className="text-sm text-yellow-800">
            Real-time updates are currently unavailable. The page will still function normally.
          </span>
        </div>
      )}

      {/* Error Display */}
      {error && (
        <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
          <div className="flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
            <div className="flex-1">
              <p className="text-red-800 font-medium">Error loading orders</p>
              <p className="text-red-700 text-sm mt-1">{error}</p>
            </div>
            <button
              onClick={handleRetry}
              className="flex items-center gap-2 px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors text-sm"
            >
              <RefreshCw size={14} />
              Retry
            </button>
          </div>
        </div>
      )}

      {/* Filter Panel */}
      <FilterPanel
        filters={filters}
        onFiltersChange={handleFiltersChange}
        onClearFilters={handleClearFilters}
        onExport={handleExport}
      />

      {/* Order Table */}
      <OrderTable
        orders={orders}
        sort={sort}
        onSortChange={handleSortChange}
        loading={loading}
        emptyMessage="No orders found"
        onClearFilters={handleClearFilters}
      />

      {/* Pagination */}
      {!loading && orders.length > 0 && (
        <PaginationController
          currentPage={pagination.page}
          pageSize={pagination.pageSize}
          totalCount={totalCount}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      )}
    </div>
  );
}
