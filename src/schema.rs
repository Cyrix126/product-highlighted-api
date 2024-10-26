// @generated automatically by Diesel CLI.

diesel::table! {
    products_highlighted (id) {
        id -> Int4,
        product_id -> Int4,
        priority -> Int2,
        enabled -> Bool,
    }
}
