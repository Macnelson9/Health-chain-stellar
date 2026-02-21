// URLStateManager - Manages synchronization between component state and URL query parameters

import { OrderFilters, SortConfig, PaginationConfig } from '../types/orders';

export class URLStateManager {
  /**
   * Encode filters to URLSearchParams
   * Converts filter state to URL query parameters
   */
  static encodeFilters(
    filters: OrderFilters,
    sort: SortConfig,
    pagination: PaginationConfig
  ): URLSearchParams {
    const params = new URLSearchParams();

    // Date range filters
    if (filters.startDate) {
      params.set('startDate', filters.startDate.toISOString().split('T')[0]);
    }
    if (filters.endDate) {
      params.set('endDate', filters.endDate.toISOString().split('T')[0]);
    }

    // Blood types filter (comma-separated)
    if (filters.bloodTypes.length > 0) {
      params.set('bloodTypes', filters.bloodTypes.join(','));
    }

    // Statuses filter (comma-separated)
    if (filters.statuses.length > 0) {
      params.set('statuses', filters.statuses.join(','));
    }

    // Blood bank search (URL-encoded automatically by URLSearchParams)
    if (filters.bloodBank) {
      params.set('bloodBank', filters.bloodBank);
    }

    // Sort configuration
    if (sort.column) {
      params.set('sortBy', sort.column);
      params.set('sortOrder', sort.order);
    }

    // Pagination
    params.set('page', pagination.page.toString());
    params.set('pageSize', pagination.pageSize.toString());

    return params;
  }

  /**
   * Decode URLSearchParams to filters
   * Parses URL query parameters back to filter state
   */
  static decodeFilters(params: URLSearchParams): {
    filters: OrderFilters;
    sort: SortConfig;
    pagination: PaginationConfig;
  } {
    const filters: OrderFilters = {
      startDate: null,
      endDate: null,
      bloodTypes: [],
      statuses: [],
      bloodBank: '',
    };

    const sort: SortConfig = {
      column: 'placedAt',
      order: 'desc',
    };

    const pagination: PaginationConfig = {
      page: 1,
      pageSize: 25,
    };

    // Parse date range
    const startDateStr = params.get('startDate');
    if (startDateStr) {
      filters.startDate = new Date(startDateStr);
    }

    const endDateStr = params.get('endDate');
    if (endDateStr) {
      filters.endDate = new Date(endDateStr);
    }

    // Parse blood types
    const bloodTypesStr = params.get('bloodTypes');
    if (bloodTypesStr) {
      filters.bloodTypes = bloodTypesStr.split(',') as any[];
    }

    // Parse statuses
    const statusesStr = params.get('statuses');
    if (statusesStr) {
      filters.statuses = statusesStr.split(',') as any[];
    }

    // Parse blood bank search
    const bloodBank = params.get('bloodBank');
    if (bloodBank) {
      filters.bloodBank = bloodBank;
    }

    // Parse sort configuration
    const sortBy = params.get('sortBy');
    if (sortBy) {
      sort.column = sortBy;
    }

    const sortOrder = params.get('sortOrder');
    if (sortOrder === 'asc' || sortOrder === 'desc') {
      sort.order = sortOrder;
    }

    // Parse pagination
    const pageStr = params.get('page');
    if (pageStr) {
      const page = parseInt(pageStr, 10);
      if (!isNaN(page) && page > 0) {
        pagination.page = page;
      }
    }

    const pageSizeStr = params.get('pageSize');
    if (pageSizeStr) {
      const pageSize = parseInt(pageSizeStr, 10);
      if (pageSize === 25 || pageSize === 50 || pageSize === 100) {
        pagination.pageSize = pageSize;
      }
    }

    return { filters, sort, pagination };
  }

  /**
   * Update browser URL without reload
   * Uses history.pushState to update URL without triggering navigation
   */
  static updateURL(
    filters: OrderFilters,
    sort: SortConfig,
    pagination: PaginationConfig
  ): void {
    const params = this.encodeFilters(filters, sort, pagination);
    const newUrl = `${window.location.pathname}?${params.toString()}`;
    window.history.pushState({}, '', newUrl);
  }

  /**
   * Read state from current URL
   * Initializes filter state from URL query parameters
   */
  static readFromURL(): {
    filters: OrderFilters;
    sort: SortConfig;
    pagination: PaginationConfig;
  } {
    if (typeof window === 'undefined') {
      // Server-side rendering - return defaults
      return {
        filters: {
          startDate: null,
          endDate: null,
          bloodTypes: [],
          statuses: [],
          bloodBank: '',
        },
        sort: {
          column: 'placedAt',
          order: 'desc',
        },
        pagination: {
          page: 1,
          pageSize: 25,
        },
      };
    }

    const params = new URLSearchParams(window.location.search);
    return this.decodeFilters(params);
  }
}
