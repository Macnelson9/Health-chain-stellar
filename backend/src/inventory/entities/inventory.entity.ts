import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  Index,
} from 'typeorm';

@Entity('inventory')
@Index(['hospitalId', 'bloodType'])
@Index(['bloodType', 'quantity'])
export class InventoryEntity {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ name: 'hospital_id' })
  @Index()
  hospitalId: string;

  @Column({ name: 'blood_type' })
  bloodType: string;

  @Column({ type: 'int', default: 0 })
  quantity: number;

  @Column({ name: 'reserved_quantity', type: 'int', default: 0 })
  reservedQuantity: number;

  @Column({ name: 'available_quantity', type: 'int', generated: 'STORED' })
  availableQuantity: number;

  @Column({ name: 'reorder_level', type: 'int', default: 10 })
  reorderLevel: number;

  @Column({ name: 'max_capacity', type: 'int', default: 100 })
  maxCapacity: number;

  @Column({ type: 'jsonb', nullable: true })
  metadata: Record<string, any>;

  @CreateDateColumn({ name: 'created_at' })
  createdAt: Date;

  @UpdateDateColumn({ name: 'updated_at' })
  updatedAt: Date;
}
