import { Order } from '../types/order.types';

export interface PaginationMetadata {
  currentPage: number;
  pageSize: number;
  totalCount: number;
  totalPages: number;
}

export interface OrdersResponseDto {
  data: Order[];
  pagination: PaginationMetadata;
}
