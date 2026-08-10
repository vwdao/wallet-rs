-- Sync cursors moved to Redis; the Postgres table is no longer used.
DROP TABLE IF EXISTS sync_cursors;
