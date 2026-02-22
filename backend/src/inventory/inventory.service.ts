import { Injectable, NotFoundException, BadRequestException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { InventoryEntity } from './entities/inventory.entity';
import { InventoryRepository } from './repositories/inventory.repository';
import { CreateInventoryDto } from './dto/create-inventory.dto';
import { UpdateInventoryDto } from './dto/update-inventory.dto';

@Injectable()
export class InventoryService {
  constructor(
    @InjectRepository(InventoryEntity)
    private readonly inventoryRepo: Repository<InventoryEntity>,
    private readonly inventoryRepository: InventoryRepository,
  ) {}

  async findAll(hospitalId?: string) {
    let items: InventoryEntity[];

    if (hospitalId) {
      items = await this.inventoryRepository.findByHospital(hospitalId);
    } else {
      items = await this.inventoryRepo.find({
        order: { bloodType: 'ASC', hospitalId: 'ASC' },
      });
    }

    return {
      message: 'Inventory items retrieved successfully',
      data: items,
    };
  }

  async findOne(id: string) {
    const item = await this.inventoryRepo.findOne({ where: { id } });

    if (!item) {
      throw new NotFoundException(`Inventory item '${id}' not found`);
    }

    return {
      message: 'Inventory item retrieved successfully',
      data: item,
    };
  }

  async create(createInventoryDto: CreateInventoryDto) {
    // Check if inventory already exists for this hospital and blood type
    const existing = await this.inventoryRepository.findByHospitalAndBloodType(
      createInventoryDto.hospitalId,
      createInventoryDto.bloodType,
    );

    if (existing) {
      throw new BadRequestException(
        `Inventory already exists for hospital ${createInventoryDto.hospitalId} and blood type ${createInventoryDto.bloodType}`,
      );
    }

    const item = this.inventoryRepo.create(createInventoryDto);
    const saved = await this.inventoryRepo.save(item);

    return {
      message: 'Inventory item created successfully',
      data: saved,
    };
  }

  async update(id: string, updateInventoryDto: UpdateInventoryDto) {
    const item = await this.inventoryRepo.findOne({ where: { id } });

    if (!item) {
      throw new NotFoundException(`Inventory item '${id}' not found`);
    }

    Object.assign(item, updateInventoryDto);
    const updated = await this.inventoryRepo.save(item);

    return {
      message: 'Inventory item updated successfully',
      data: updated,
    };
  }

  async remove(id: string) {
    const item = await this.inventoryRepo.findOne({ where: { id } });

    if (!item) {
      throw new NotFoundException(`Inventory item '${id}' not found`);
    }

    await this.inventoryRepo.remove(item);

    return {
      message: 'Inventory item deleted successfully',
      data: { id },
    };
  }

  async updateStock(id: string, quantity: number) {
    const item = await this.inventoryRepo.findOne({ where: { id } });

    if (!item) {
      throw new NotFoundException(`Inventory item '${id}' not found`);
    }

    if (quantity < 0 && Math.abs(quantity) > item.quantity) {
      throw new BadRequestException(
        `Cannot reduce stock by ${Math.abs(quantity)}. Current quantity is ${item.quantity}`,
      );
    }

    await this.inventoryRepository.adjustStock(id, quantity);

    const updated = await this.inventoryRepo.findOne({ where: { id } });

    return {
      message: 'Stock updated successfully',
      data: updated,
    };
  }

  async getLowStockItems(threshold: number = 10) {
    const items = await this.inventoryRepository.getLowStockItems(threshold);

    return {
      message: 'Low stock items retrieved successfully',
      data: items,
    };
  }

  async getCriticalStockItems() {
    const items = await this.inventoryRepository.getCriticalStockItems();

    return {
      message: 'Critical stock items retrieved successfully',
      data: items,
    };
  }

  async getStockAggregation() {
    const aggregation =
      await this.inventoryRepository.getStockAggregationByBloodType();

    return {
      message: 'Stock aggregation retrieved successfully',
      data: aggregation,
    };
  }

  async getInventoryStats(hospitalId?: string) {
    const stats = await this.inventoryRepository.getInventoryStats(hospitalId);

    return {
      message: 'Inventory statistics retrieved successfully',
      data: stats,
    };
  }

  async getReorderSummary() {
    const summary = await this.inventoryRepository.getReorderSummary();

    return {
      message: 'Reorder summary retrieved successfully',
      data: summary,
    };
  }

  async reserveStock(id: string, quantity: number) {
    const success = await this.inventoryRepository.reserveStock(id, quantity);

    if (!success) {
      throw new BadRequestException(
        `Insufficient available stock to reserve ${quantity} units`,
      );
    }

    const updated = await this.inventoryRepo.findOne({ where: { id } });

    return {
      message: 'Stock reserved successfully',
      data: updated,
    };
  }

  async releaseStock(id: string, quantity: number) {
    await this.inventoryRepository.releaseStock(id, quantity);

    const updated = await this.inventoryRepo.findOne({ where: { id } });

    return {
      message: 'Stock released successfully',
      data: updated,
    };
  }
}
