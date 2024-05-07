use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

use crate::repositories::schema::users;
#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub email: String,
    pub encrypted_password: String,
    pub reset_password_token: String,
    pub reset_password_sent_at: Option<chrono::NaiveDateTime>,
    pub remember_created_at: Option<chrono::NaiveDateTime>,
    pub sign_in_count: Option<Numeric>,
    pub current_sign_in_at: Option<chrono::NaiveDateTime>,
    pub last_sign_in_at: Option<chrono::NaiveDateTime>,
    pub current_sign_in_ip: Option<Inet>,
    pub last_sign_in_ip: Option<Inet>,
    pub status: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
