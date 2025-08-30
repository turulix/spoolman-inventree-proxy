CREATE TABLE pending_spool_usage
(
    spool_id        INT  NOT NULL PRIMARY KEY,
    pending_weight  REAL NOT NULL,
    last_updated_at TEXT NOT NULL
);

