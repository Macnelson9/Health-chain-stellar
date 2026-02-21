// PaginationController - Manages pagination controls and page size selection

import React from 'react';
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-react';

interface PaginationControllerProps {
  currentPage: number;
  pageSize: number;
  totalCount: number;
  onPageChange: (page: number) => void;
  onPageSizeChange: (size: number) => void;
}

/**
 * PaginationController Component
 * Provides pagination controls and page size selection
 */
export const PaginationController: React.FC<PaginationControllerProps> = ({
  currentPage,
  pageSize,
  totalCount,
  onPageChange,
  onPageSizeChange,
}) => {
  // Calculate total pages
  const totalPages = Math.ceil(totalCount / pageSize);

  // Calculate current range
  const startIndex = (currentPage - 1) * pageSize + 1;
  const endIndex = Math.min(currentPage * pageSize, totalCount);

  // Disable states
  const isFirstPage = currentPage === 1;
  const isLastPage = currentPage === totalPages || totalPages === 0;

  // Navigation handlers
  const handleFirstPage = () => {
    if (!isFirstPage) {
      onPageChange(1);
    }
  };

  const handlePreviousPage = () => {
    if (!isFirstPage) {
      onPageChange(currentPage - 1);
    }
  };

  const handleNextPage = () => {
    if (!isLastPage) {
      onPageChange(currentPage + 1);
    }
  };

  const handleLastPage = () => {
    if (!isLastPage) {
      onPageChange(totalPages);
    }
  };

  const handlePageSizeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newSize = parseInt(e.target.value, 10);
    if (newSize === 25 || newSize === 50 || newSize === 100) {
      onPageSizeChange(newSize);
    }
  };

  return (
    <div className="flex flex-col sm:flex-row items-center justify-between gap-4 py-4 px-2">
      {/* Item range display */}
      <div className="text-sm text-gray-600">
        {totalCount > 0 ? (
          <>
            Showing <span className="font-medium">{startIndex}</span> to{' '}
            <span className="font-medium">{endIndex}</span> of{' '}
            <span className="font-medium">{totalCount}</span> orders
          </>
        ) : (
          'No orders to display'
        )}
      </div>

      {/* Pagination controls */}
      <div className="flex items-center gap-4">
        {/* Page size selector */}
        <div className="flex items-center gap-2">
          <label htmlFor="pageSize" className="text-sm text-gray-600">
            Rows per page:
          </label>
          <select
            id="pageSize"
            value={pageSize}
            onChange={handlePageSizeChange}
            className="border border-gray-300 rounded px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value={25}>25</option>
            <option value={50}>50</option>
            <option value={100}>100</option>
          </select>
        </div>

        {/* Page navigation */}
        <div className="flex items-center gap-1">
          {/* First page button */}
          <button
            onClick={handleFirstPage}
            disabled={isFirstPage}
            className={`p-1 rounded ${
              isFirstPage
                ? 'text-gray-300 cursor-not-allowed'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
            aria-label="First page"
          >
            <ChevronsLeft size={20} />
          </button>

          {/* Previous page button */}
          <button
            onClick={handlePreviousPage}
            disabled={isFirstPage}
            className={`p-1 rounded ${
              isFirstPage
                ? 'text-gray-300 cursor-not-allowed'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
            aria-label="Previous page"
          >
            <ChevronLeft size={20} />
          </button>

          {/* Page indicator */}
          <span className="px-3 text-sm text-gray-600">
            Page <span className="font-medium">{currentPage}</span> of{' '}
            <span className="font-medium">{totalPages || 1}</span>
          </span>

          {/* Next page button */}
          <button
            onClick={handleNextPage}
            disabled={isLastPage}
            className={`p-1 rounded ${
              isLastPage
                ? 'text-gray-300 cursor-not-allowed'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
            aria-label="Next page"
          >
            <ChevronRight size={20} />
          </button>

          {/* Last page button */}
          <button
            onClick={handleLastPage}
            disabled={isLastPage}
            className={`p-1 rounded ${
              isLastPage
                ? 'text-gray-300 cursor-not-allowed'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
            aria-label="Last page"
          >
            <ChevronsRight size={20} />
          </button>
        </div>
      </div>
    </div>
  );
};
