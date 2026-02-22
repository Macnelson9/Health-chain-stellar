import { MigrationInterface, QueryRunner, Table, TableIndex } from 'typeorm';

export class CreateInventoryTable1740000000000 implements MigrationInterface {
  public async up(queryRunner: QueryRunner): Promise<void> {
    await queryRunner.createTable(
      new Table({
        name: 'inventory',
        columns: [
          {
            name: 'id',
            type: 'uuid',
            isPrimary: true,
            generationStrategy: 'uuid',
            default: 'uuid_generate_v4()',
          },
          {
            name: 'hospital_id',
            type: 'varchar',
            isNullable: false,
          },
          {
            name: 'blood_type',
            type: 'varchar',
            isNullable: false,
          },
          {
            name: 'quantity',
            type: 'int',
            default: 0,
          },
          {
            name: 'reserved_quantity',
            type: 'int',
            default: 0,
          },
          {
            name: 'available_quantity',
            type: 'int',
            generatedType: 'STORED',
            asExpression: 'quantity - reserved_quantity',
          },
          {
            name: 'reorder_level',
            type: 'int',
            default: 10,
          },
          {
            name: 'max_capacity',
            type: 'int',
            default: 100,
          },
          {
            name: 'metadata',
            type: 'jsonb',
            isNullable: true,
          },
          {
            name: 'created_at',
            type: 'timestamp',
            default: 'CURRENT_TIMESTAMP',
          },
          {
            name: 'updated_at',
            type: 'timestamp',
            default: 'CURRENT_TIMESTAMP',
          },
        ],
      }),
      true,
    );

    // Create composite index for hospital_id + blood_type (unique constraint)
    await queryRunner.createIndex(
      'inventory',
      new TableIndex({
        name: 'idx_inventory_hospital_blood_type',
        columnNames: ['hospital_id', 'blood_type'],
        isUnique: true,
      }),
    );

    // Create index for blood_type + quantity (for aggregations and low stock queries)
    await queryRunner.createIndex(
      'inventory',
      new TableIndex({
        name: 'idx_inventory_blood_type_quantity',
        columnNames: ['blood_type', 'quantity'],
      }),
    );

    // Create index for hospital_id (for filtering by hospital)
    await queryRunner.createIndex(
      'inventory',
      new TableIndex({
        name: 'idx_inventory_hospital_id',
        columnNames: ['hospital_id'],
      }),
    );

    // Create index for low stock queries
    await queryRunner.createIndex(
      'inventory',
      new TableIndex({
        name: 'idx_inventory_quantity_reorder',
        columnNames: ['quantity', 'reorder_level'],
      }),
    );
  }

  public async down(queryRunner: QueryRunner): Promise<void> {
    await queryRunner.dropTable('inventory');
  }
}
