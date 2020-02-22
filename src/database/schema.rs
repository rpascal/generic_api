table! {
    user_endpoints (id) {
        key -> Uuid,
        endpoint -> Text,
        response -> Json,
        id -> Int4,
    }
}
