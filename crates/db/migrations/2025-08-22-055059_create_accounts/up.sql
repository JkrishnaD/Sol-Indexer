-- Your SQL goes here
CREATE TABLE accounts(
    id BIGSERIAL PRIMARY KEY,
    block_id BIGINT NOT NULL REFERENCES blocks(id) ON DELETE CASCADE,
    slot BIGINT NOT NULL,
    pubkey TEXT NOT NULL,
    lamports BIGINT NOT NULL,
    owner TEXT NOT NULL,
    executable BOOLEAN NOT NULL,
    rent_epoch BIGINT NOT NULL,
    data BYTEA NOT NULL,
    write_version BIGINT NOT NULL,
    txn_signature TEXT
)