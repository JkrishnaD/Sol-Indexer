-- Your SQL goes here
CREATE TABLE transaction_meta (
    id BIGSERIAL PRIMARY KEY,
    transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    fee BIGINT NOT NULL,
    compute_units_consumed BIGINT,
    pre_balances BIGINT[] NOT NULL,
    post_balances BIGINT[] NOT NULL,
    log_messages TEXT[] NOT NULL
);