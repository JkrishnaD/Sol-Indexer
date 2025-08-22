-- Your SQL goes here
CREATE TABLE token_balances (
    id BIGSERIAL PRIMARY KEY,
    transaction_meta_id BIGINT NOT NULL REFERENCES transaction_meta(id) ON DELETE CASCADE,
    account_index INT NOT NULL,
    mint TEXT NOT NULL,
    owner TEXT NOT NULL,
    program_id TEXT NOT NULL,
    balance_type TEXT NOT NULL CHECK (balance_type IN ('pre', 'post'))
);