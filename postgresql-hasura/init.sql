-- Initialize PostgreSQL database for GraphQL benchmarking
-- This mirrors the structure used in the existing benchmarks

CREATE TABLE IF NOT EXISTS links (
    id BIGSERIAL PRIMARY KEY,
    source BIGINT NOT NULL,
    target BIGINT NOT NULL
);

-- Create indexes for performance (similar to existing benchmarks)
CREATE INDEX IF NOT EXISTS idx_links_source ON links(source);
CREATE INDEX IF NOT EXISTS idx_links_target ON links(target);
CREATE INDEX IF NOT EXISTS idx_links_source_target ON links(source, target);

-- Insert 3000 background links to match existing benchmark conditions
INSERT INTO links (source, target) 
SELECT 
    (random() * 1000)::int + 1,
    (random() * 1000)::int + 1
FROM generate_series(1, 3000);

-- Create function for point links (where id = source = target)
CREATE OR REPLACE FUNCTION create_point_link() RETURNS links AS $$
DECLARE
    new_link links;
BEGIN
    INSERT INTO links (source, target) VALUES (0, 0) RETURNING * INTO new_link;
    UPDATE links SET source = new_link.id, target = new_link.id WHERE id = new_link.id RETURNING * INTO new_link;
    RETURN new_link;
END;
$$ LANGUAGE plpgsql;