use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, Selectable};
use uuid::Uuid;

use crate::db::schema::users;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = users)]
pub struct User {  
    #[serde(default)]
    // #[serde(skip_serializing)]
    pub id: Uuid,
    pub email: String,
    pub encrypted_password: String,
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<chrono::NaiveDateTime>,
    pub remember_created_at: Option<chrono::NaiveDateTime>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<chrono::NaiveDateTime>,
    pub last_sign_in_at: Option<chrono::NaiveDateTime>,   
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub status: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: NaiveDateTime,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[serde(default)]   
    pub email: String,
    pub encrypted_password: String,
    // #[serde(skip_serializing)]
    // pub reset_password_token: Option<String>,
    // #[serde(skip_serializing)]
    // pub reset_password_sent_at: Option<chrono::NaiveDateTime>,
    // #[serde(skip_serializing)]
    // pub remember_created_at: Option<chrono::NaiveDateTime>,
    // // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(skip_serializing)]
    // pub sign_in_count: i32,
    // #[serde(skip_serializing)]
    pub current_sign_in_at: Option<chrono::NaiveDateTime>,
    // #[serde(skip_serializing)]
    pub last_sign_in_at: Option<chrono::NaiveDateTime>,
    // #[serde(skip_serializing)]
    pub current_sign_in_ip: Option<String>,
    // #[serde(skip_serializing)]
    pub last_sign_in_ip: Option<String>,
    // #[serde(skip_serializing)]
    // pub status: Option<bool>,
}

// #[diesel(postgres_type(name = "inet"))] bug diesel not working type inet postgresql
