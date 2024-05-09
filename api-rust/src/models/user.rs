use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use uuid::Uuid;

use crate::db::schema::users;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {  
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    pub encrypted_password: String,
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<chrono::NaiveDateTime>,
    pub remember_created_at: Option<chrono::NaiveDateTime>,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<chrono::NaiveDateTime>,
    pub last_sign_in_at: Option<chrono::NaiveDateTime>,   
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub status: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// #[diesel(postgres_type(name = "inet"))] bug diesel not working type inet postgresql
