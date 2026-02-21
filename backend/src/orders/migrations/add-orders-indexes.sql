-- Migration: Add indexes for Hospital Order History Dashboard performance
-- This migration should be run when a database is configured

-- Index for filtering by hospital
CREATE INDEX IF NOT EXISTS idx_orders_hospital_id ON orders(hospital_id);

-- Index for date range filtering and sorting
CREATE INDEX IF NOT EXISTS idx_orders_placed_at ON orders(placed_at);

-- Index for status filtering
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- Index for blood type filtering
CREATE INDEX IF NOT EXISTS idx_orders_blood_type ON orders(blood_type);

-- Composite index for common query pattern (hospital + date)
CREATE INDEX IF NOT EXISTS idx_orders_hospital_placed_at ON orders(hospital_id, placed_at);

-- Composite index for active orders queries (hospital + status)
CREATE INDEX IF NOT EXISTS idx_orders_hospital_status ON orders(hospital_id, status);

-- Index for blood bank name search (if using PostgreSQL with text search)
-- For case-insensitive partial matching
CREATE INDEX IF NOT EXISTS idx_orders_blood_bank_name ON orders(blood_bank_name);
-- Or for PostgreSQL with trigram extension for better LIKE performance:
-- CREATE INDEX IF NOT EXISTS idx_orders_blood_bank_name_trgm ON orders USING gin(blood_bank_name gin_trgm_ops);
