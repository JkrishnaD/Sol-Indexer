-- Your SQL goes here
CREATE TABLE entries (
    id BIGSERIAL PRIMARY KEY,
    block_id BIGINT NOT NULL REFERENCES blocks(id) ON DELETE CASCADE,
    slot BIGINT NOT NULL,
    entry_index BIGINT NOT NULL,
    num_hashes BIGINT NOT NULL,
    hash BYTEA NOT NULL,
    executed_transaction_count BIGINT NOT NULL,
    starting_transaction_index BIGINT NOT NULL
);
