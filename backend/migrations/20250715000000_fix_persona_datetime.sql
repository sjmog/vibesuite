-- Fix last_quota_reset to store full datetime instead of just date
-- This fixes the "invalid datetime" error when parsing dates like "2025-07-15"

-- Update existing date values to datetime format
UPDATE project_personas 
SET last_quota_reset = last_quota_reset || 'T00:00:00.000Z'
WHERE last_quota_reset NOT LIKE '%T%';

-- Also update the default value for future inserts
-- Note: SQLite doesn't support ALTER COLUMN directly, but the default is only used on INSERT
-- The existing default will work but we should ensure consistency in application code