// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        block_id -> Int8,
        slot -> Int8,
        pubkey -> Text,
        lamports -> Int8,
        owner -> Text,
        executable -> Bool,
        rent_epoch -> Int8,
        data -> Bytea,
        write_version -> Int8,
        txn_signature -> Nullable<Text>,
    }
}

diesel::table! {
    blocks (id) {
        id -> Int8,
        slot -> Int8,
        blockhash -> Text,
        parent_slot -> Int8,
        block_time -> Timestamp,
    }
}

diesel::table! {
    entries (id) {
        id -> Int8,
        block_id -> Int8,
        slot -> Int8,
        entry_index -> Int8,
        num_hashes -> Int8,
        hash -> Bytea,
        executed_transaction_count -> Int8,
        starting_transaction_index -> Int8,
    }
}

diesel::table! {
    token_balances (id) {
        id -> Int8,
        transaction_meta_id -> Int8,
        account_index -> Int4,
        mint -> Text,
        owner -> Text,
        program_id -> Text,
        balance_type -> Text,
    }
}

diesel::table! {
    transaction_meta (id) {
        id -> Int8,
        transaction_id -> Int8,
        fee -> Int8,
        compute_units_consumed -> Nullable<Int8>,
        pre_balances -> Array<Nullable<Int8>>,
        post_balances -> Array<Nullable<Int8>>,
        log_messages -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int8,
        block_id -> Int8,
        signature -> Text,
        is_vote -> Bool,
        tx_index -> Int8,
    }
}

diesel::joinable!(accounts -> blocks (block_id));
diesel::joinable!(entries -> blocks (block_id));
diesel::joinable!(token_balances -> transaction_meta (transaction_meta_id));
diesel::joinable!(transaction_meta -> transactions (transaction_id));
diesel::joinable!(transactions -> blocks (block_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    blocks,
    entries,
    token_balances,
    transaction_meta,
    transactions,
);
