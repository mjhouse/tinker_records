// @generated automatically by Diesel CLI.

diesel::table! {
    abilities (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        username -> Text,
        created -> Timestamptz,
        x -> Float4,
        y -> Float4,
        modified -> Timestamptz,
        password -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    join_character_abilities (character_id, ability_id) {
        character_id -> Int4,
        ability_id -> Int4,
    }
}

diesel::table! {
    join_character_items (character_id, item_id) {
        character_id -> Int4,
        item_id -> Int4,
    }
}

diesel::joinable!(join_character_abilities -> abilities (ability_id));
diesel::joinable!(join_character_abilities -> characters (character_id));
diesel::joinable!(join_character_items -> characters (character_id));
diesel::joinable!(join_character_items -> items (item_id));

diesel::allow_tables_to_appear_in_same_query!(
    abilities,
    characters,
    items,
    join_character_abilities,
    join_character_items,
);
