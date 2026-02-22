import { IsString, IsInt, IsOptional, Min, IsObject } from 'class-validator';

export class CreateInventoryDto {
  @IsString()
  hospitalId: string;

  @IsString()
  bloodType: string;

  @IsInt()
  @Min(0)
  quantity: number;

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
