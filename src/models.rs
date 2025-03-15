use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ------------------------------------------------
// Characters
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterInsert {
    pub username: String
}

#[derive(Queryable, Selectable, Clone, Serialize, Deserialize, Default, Debug)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterSelect {
    pub id: i32,
    pub username: String,
    pub created: DateTime<Utc>,
    pub x: f32,
    pub y: f32,
    pub modified: DateTime<Utc>,
    pub password: String,
}
// ------------------------------------------------

// ------------------------------------------------
// Abilities
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::abilities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AbilityInsert {
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::abilities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AbilitySelect {
    pub id: i32,
    pub name: String,
}
// ------------------------------------------------

// ------------------------------------------------
// Items
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemInsert {
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemSelect {
    pub id: i32,
    pub name: String,
}
// ------------------------------------------------