-- Rapid Download Manager - Database Schema Migration v1
-- Engine: SQLite / Atomic JSON Storage with dual .bak resilience

CREATE TABLE IF NOT EXISTS download_history (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    target_file TEXT NOT NULL,
    total_bytes INTEGER,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    completed_at TEXT NOT NULL,
    speed_bps INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_history_status ON download_history(status);
CREATE INDEX IF NOT EXISTS idx_history_date ON download_history(completed_at);

CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS scheduler_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    enabled INTEGER NOT NULL DEFAULT 0,
    start_hour INTEGER NOT NULL DEFAULT 2,
    start_minute INTEGER NOT NULL DEFAULT 0,
    stop_hour INTEGER NOT NULL DEFAULT 7,
    stop_minute INTEGER NOT NULL DEFAULT 30,
    auto_shutdown INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
