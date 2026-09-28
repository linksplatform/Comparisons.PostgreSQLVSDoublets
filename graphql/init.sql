-- Dedicated disposable benchmark database, never applied to an existing service.
CREATE TABLE links (
    id BIGSERIAL PRIMARY KEY,
    from_id BIGINT NOT NULL,
    to_id BIGINT NOT NULL,
    UNIQUE (from_id, to_id)
);
CREATE INDEX links_from_id_idx ON links (from_id);
CREATE INDEX links_to_id_idx ON links (to_id);
