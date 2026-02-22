import { IsInt, IsOptional, Min, IsObject } from 'class-validator';

export class UpdateInventoryDto {
  @IsInt()
  @Min(0)
  @IsOptional()
  quantity?: number;

  @IsInt()
  @Min(0)
  @IsOptional()
  reorderLevel?: number;

  @IsInt()
  @Min(0)
  @IsOptional()
  maxCapacity?: number;

  @IsObject()
  @IsOptional()
  metadata?: Record<string, any>;
}
