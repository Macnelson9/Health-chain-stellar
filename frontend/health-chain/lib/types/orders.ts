// Type definitions for Hospital Order History Dashboard

export type BloodType = 'A+' | 'A-' | 'B+' | 'B-' | 'AB+' | 'AB-' | 'O+' | 'O-';

export type OrderStatus = 'pending' | 'confirmed' | 'in_transit' | 'delivered' | 'cancelled';

export interface BloodBankInfo {
  id: string;
  name: string;
  location: string;
}

export interface HospitalInfo {
  id: string;
  name: string;
  location: string;
}

export interface RiderInfo {
  id: string;
  name: string;
  phone: string;
}

export interface Order {
  id: string;
  bloodType: BloodType;
  quantity: number;
  bloodBank: BloodBankInfo;
  hospital: HospitalInfo;
  status: OrderStatus;
  rider: RiderInfo | null;
  placedAt: Date;
  deliveredAt: Date | null;
  confirmedAt: Date | null;
  cancelledAt: Date | null;
  createdAt: Date;
  updatedAt: Date;
}

export interface OrderFilters {
  startDate: Date | null;
  endDate: Date | null;
  bloodTypes: BloodType[];
  statuses: OrderStatus[];
  bloodBank: string;
}

export interface SortConfig {
  column: string;
  order: 'asc' | 'desc';
}

export interface PaginationConfig {
  page: number;
  pageSize: 25 | 50 | 100;
}

export interface OrdersResponse {
  data: Order[];
  pagination: {
    currentPage: number;
    pageSize: number;
    totalCount: number;
    totalPages: number;
  };
}

export interface OrderQueryParams extends OrderFilters, SortConfig, PaginationConfig {
  hospitalId: string;
}
