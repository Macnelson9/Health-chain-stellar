// StatusBadge - Displays order status with color coding and icons

import React from 'react';
import { Clock, CheckCircle, Truck, CheckCircle2, XCircle } from 'lucide-react';
import { OrderStatus } from '@/lib/types/orders';

interface StatusBadgeProps {
  status: OrderStatus;
  size?: 'sm' | 'md' | 'lg';
}

/**
 * StatusBadge Component
 * Displays order status with appropriate color and icon
 * 
 * Status color mapping:
 * - pending: Amber (#FFA500)
 * - confirmed: Blue (#5C9CE0)
 * - in_transit: Purple (#C68AF2)
 * - delivered: Green (#00BFA5)
 * - cancelled: Red (#D32F2F)
 */
export const StatusBadge: React.FC<StatusBadgeProps> = ({ status, size = 'md' }) => {
  // Status to color mapping
  const colorClasses = {
    pending: 'bg-amber-100 text-amber-700 border-amber-300',
    confirmed: 'bg-blue-100 text-blue-700 border-blue-300',
    in_transit: 'bg-purple-100 text-purple-700 border-purple-300',
    delivered: 'bg-green-100 text-green-700 border-green-300',
    cancelled: 'bg-red-100 text-red-700 border-red-300',
  };

  // Status to icon mapping
  const StatusIcon = {
    pending: Clock,
    confirmed: CheckCircle,
    in_transit: Truck,
    delivered: CheckCircle2,
    cancelled: XCircle,
  }[status];

  // Size classes
  const sizeClasses = {
    sm: 'px-2 py-1 text-xs gap-1',
    md: 'px-3 py-1.5 text-sm gap-1.5',
    lg: 'px-4 py-2 text-base gap-2',
  };

  const iconSizes = {
    sm: 12,
    md: 14,
    lg: 16,
  };

  // Format status text (capitalize and replace underscores)
  const statusText = status
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');

  return (
    <span
      className={`inline-flex items-center font-medium rounded-full border ${colorClasses[status]} ${sizeClasses[size]}`}
    >
      <StatusIcon size={iconSizes[size]} />
      <span>{statusText}</span>
    </span>
  );
};
