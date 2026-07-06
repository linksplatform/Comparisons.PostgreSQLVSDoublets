CREATE TABLE IF NOT EXISTS entities (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    value INTEGER NOT NULL
);

CREATE INDEX idx_entities_value ON entities(value);

INSERT INTO entities (name, value) SELECT 'entity_' || i, i FROM generate_series(1, 10000) i;
