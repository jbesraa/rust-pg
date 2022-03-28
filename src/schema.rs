table! {
    users (id) {
        id -> Int4,
        user_info -> Varchar,
        username -> Varchar,
        wallet_address -> Varchar,
        social_networks -> Jsonb,
    }
}
