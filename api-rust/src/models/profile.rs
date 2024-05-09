use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, Associations};
use uuid::Uuid;

use crate::db::schema::profiles;

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable,  Associations, Insertable, AsChangeset )]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profiles)]
pub struct Profile {
    #[serde(default)]
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub birthday: chrono::NaiveDate,
    pub avatar: String, 
    pub phone: Option<i64>,
    pub user_id: Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
