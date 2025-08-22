-- Your SQL goes here
CREATE TABLE blocks (
    id BIGSERIAL PRIMARY KEY,
    slot BIGINT NOT NULL UNIQUE,
    blockhash TEXT NOT NULL UNIQUE,
    parent_slot BIGINT NOT NULL,
    parent_blockhash TEXT NOT NULL,
    executed_transaction_count BIGINT NOT NULL,
    updated_account_count BIGINT NOT NULL,
    entries_count BIGINT NOT NULL,
    block_time TIMESTAMP DEFAULT NOW()
);
