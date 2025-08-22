-- Your SQL goes here
CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    block_id BIGINT NOT NULL REFERENCES blocks(id) ON DELETE CASCADE,
    signature TEXT NOT NULL UNIQUE,
    is_vote BOOLEAN NOT NULL,
    tx_index BIGINT NOT NULL
);