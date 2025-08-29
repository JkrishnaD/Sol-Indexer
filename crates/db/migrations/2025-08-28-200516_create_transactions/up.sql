-- Your SQL goes here       
CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    slot BIGINT NOT NULL,
    signature BYTEA NOT NULL,
    is_vote BOOLEAN NOT NULL,
    idx INT NOT NULL,
    fee BIGINT,
    compute_units_consumed BIGINT,
    pre_balances BIGINT[],
    post_balances BIGINT[],
    log_messages TEXT[],
    pre_token_balances BIGINT[],
    post_token_balances BIGINT[]
);
