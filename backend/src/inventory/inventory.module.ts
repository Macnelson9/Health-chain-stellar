import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { InventoryService } from './inventory.service';
import { InventoryController } from './inventory.controller';
import { InventoryEntity } from './entities/inventory.entity';
import { InventoryRepository } from './repositories/inventory.repository';

@Module({
  imports: [TypeOrmModule.forFeature([InventoryEntity])],
  controllers: [InventoryController],
  providers: [InventoryService, InventoryRepository],
  exports: [InventoryService, InventoryRepository],
})
export class InventoryModule {}
