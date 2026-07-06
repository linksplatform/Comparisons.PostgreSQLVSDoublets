CREATE TABLE IF NOT EXISTS link (
    id SERIAL PRIMARY KEY,
    source INTEGER NOT NULL,
    target INTEGER NOT NULL,
    type INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

INSERT INTO link (source, target, type) VALUES
(1, 2, 1),
(2, 3, 1),
(3, 4, 2),
(4, 5, 2),
(1, 5, 3);

-- For more realistic data, use a script to insert more rows.