table! {
    blocks (height) {
        height -> Numeric,
        hash -> Varchar,
        prev_hash -> Varchar,
        timestamp -> Numeric,
        total_supply -> Numeric,
        gas_limit -> Numeric,
        gas_used -> Numeric,
        gas_price -> Numeric,
    }
}

table! {
    chunks (hash) {
        block_id -> Numeric,
        hash -> Varchar,
        shard_id -> Numeric,
        signature -> Text,
        gas_limit -> Numeric,
        gas_used -> Numeric,
        height_created -> Numeric,
        height_included -> Numeric,
    }
}

joinable!(chunks -> blocks (block_id));

allow_tables_to_appear_in_same_query!(
    blocks,
    chunks,
);
