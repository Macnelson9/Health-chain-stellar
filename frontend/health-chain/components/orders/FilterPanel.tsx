// FilterPanel - Displays all filter controls and manages filter state

import React, { useState, useEffect, useCallback } from 'react';
import { Calendar, Filter, Download, X } from 'lucide-react';
import { OrderFilters, BloodType, OrderStatus } from '@/lib/types/orders';

interface FilterPanelProps {
  filters: OrderFilters;
  onFiltersChange: (filters: OrderFilters) => void;
  onClearFilters: () => void;
  onExport: () => void;
}

const BLOOD_TYPES: BloodType[] = ['A+', 'A-', 'B+', 'B-', 'AB+', 'AB-', 'O+', 'O-'];
const STATUSES: OrderStatus[] = ['pending', 'confirmed', 'in_transit', 'delivered', 'cancelled'];

/**
 * FilterPanel Component
 * Provides filter controls for orders dashboard
 */
export const FilterPanel: React.FC<FilterPanelProps> = ({
  filters,
  onFiltersChange,
  onClearFilters,
  onExport,
}) => {
  const [bloodBankInput, setBloodBankInput] = useState(filters.bloodBank);
  const [bloodTypeOpen, setBloodTypeOpen] = useState(false);
  const [statusOpen, setStatusOpen] = useState(false);

  // Debounce blood bank search
  useEffect(() => {
    const timer = setTimeout(() => {
      if (bloodBankInput !== filters.bloodBank) {
        onFiltersChange({ ...filters, bloodBank: bloodBankInput });
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [bloodBankInput]);

  // Calculate active filter count
  const getActiveFilterCount = useCallback(() => {
    let count = 0;
    if (filters.startDate) count++;
    if (filters.endDate) count++;
    if (filters.bloodTypes.length > 0) count++;
    if (filters.statuses.length > 0) count++;
    if (filters.bloodBank) count++;
    return count;
  }, [filters]);

  const activeFilterCount = getActiveFilterCount();

  // Handle date change
  const handleDateChange = (field: 'startDate' | 'endDate', value: string) => {
    const date = value ? new Date(value) : null;
    onFiltersChange({ ...filters, [field]: date });
  };

  // Handle blood type toggle
  const handleBloodTypeToggle = (bloodType: BloodType) => {
    const newBloodTypes = filters.bloodTypes.includes(bloodType)
      ? filters.bloodTypes.filter((bt) => bt !== bloodType)
      : [...filters.bloodTypes, bloodType];
    onFiltersChange({ ...filters, bloodTypes: newBloodTypes });
  };

  // Handle status toggle
  const handleStatusToggle = (status: OrderStatus) => {
    const newStatuses = filters.statuses.includes(status)
      ? filters.statuses.filter((s) => s !== status)
      : [...filters.statuses, status];
    onFiltersChange({ ...filters, statuses: newStatuses });
  };

  // Format date for input
  const formatDateForInput = (date: Date | null) => {
    if (!date) return '';
    const d = new Date(date);
    return d.toISOString().split('T')[0];
  };

  // Format status text
  const formatStatusText = (status: OrderStatus) => {
    return status
      .split('_')
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  };

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <div className="flex flex-col lg:flex-row gap-4">
        {/* Date Range Filter */}
        <div className="flex-1">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            <Calendar size={16} className="inline mr-1" />
            Date Range
          </label>
          <div className="flex gap-2">
            <input
              type="date"
              value={formatDateForInput(filters.startDate)}
              onChange={(e) => handleDateChange('startDate', e.target.value)}
              className="flex-1 border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Start date"
            />
            <span className="self-center text-gray-500">to</span>
            <input
              type="date"
              value={formatDateForInput(filters.endDate)}
              onChange={(e) => handleDateChange('endDate', e.target.value)}
              className="flex-1 border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="End date"
            />
          </div>
        </div>

        {/* Blood Type Filter */}
        <div className="flex-1 relative">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Blood Type
          </label>
          <button
            onClick={() => setBloodTypeOpen(!bloodTypeOpen)}
            className="w-full border border-gray-300 rounded px-3 py-2 text-sm text-left focus:outline-none focus:ring-2 focus:ring-blue-500 flex justify-between items-center"
          >
            <span className="text-gray-700">
              {filters.bloodTypes.length > 0
                ? `${filters.bloodTypes.length} selected`
                : 'All blood types'}
            </span>
            <Filter size={16} />
          </button>
          {bloodTypeOpen && (
            <div className="absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded shadow-lg max-h-60 overflow-auto">
              {BLOOD_TYPES.map((bloodType) => (
                <label
                  key={bloodType}
                  className="flex items-center px-3 py-2 hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={filters.bloodTypes.includes(bloodType)}
                    onChange={() => handleBloodTypeToggle(bloodType)}
                    className="mr-2"
                  />
                  <span className="text-sm">{bloodType}</span>
                </label>
              ))}
            </div>
          )}
        </div>

        {/* Status Filter */}
        <div className="flex-1 relative">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Status
          </label>
          <button
            onClick={() => setStatusOpen(!statusOpen)}
            className="w-full border border-gray-300 rounded px-3 py-2 text-sm text-left focus:outline-none focus:ring-2 focus:ring-blue-500 flex justify-between items-center"
          >
            <span className="text-gray-700">
              {filters.statuses.length > 0
                ? `${filters.statuses.length} selected`
                : 'All statuses'}
            </span>
            <Filter size={16} />
          </button>
          {statusOpen && (
            <div className="absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded shadow-lg max-h-60 overflow-auto">
              {STATUSES.map((status) => (
                <label
                  key={status}
                  className="flex items-center px-3 py-2 hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={filters.statuses.includes(status)}
                    onChange={() => handleStatusToggle(status)}
                    className="mr-2"
                  />
                  <span className="text-sm">{formatStatusText(status)}</span>
                </label>
              ))}
            </div>
          )}
        </div>

        {/* Blood Bank Search */}
        <div className="flex-1">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Blood Bank
          </label>
          <input
            type="text"
            value={bloodBankInput}
            onChange={(e) => setBloodBankInput(e.target.value)}
            placeholder="Search blood bank..."
            className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex flex-wrap items-center gap-3 mt-4">
        {/* Active Filter Count Badge */}
        {activeFilterCount > 0 && (
          <div className="flex items-center gap-2 px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm font-medium">
            <Filter size={14} />
            <span>{activeFilterCount} active filter{activeFilterCount !== 1 ? 's' : ''}</span>
          </div>
        )}

        {/* Clear Filters Button */}
        {activeFilterCount > 0 && (
          <button
            onClick={onClearFilters}
            className="flex items-center gap-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X size={16} />
            Clear All Filters
          </button>
        )}

        {/* Export Button */}
        <button
          onClick={onExport}
          className="ml-auto flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm font-medium"
        >
          <Download size={16} />
          Export CSV
        </button>
      </div>
    </div>
  );
};
