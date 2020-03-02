table! {
    api_keys (api_key) {
        api_key -> Uuid,
    }
}

table! {
    get_requests (api_key, route) {
        api_key -> Uuid,
        route -> Text,
        response -> Jsonb,
    }
}

joinable!(get_requests -> api_keys (api_key));

allow_tables_to_appear_in_same_query!(
    api_keys,
    get_requests,
);
