use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Associations, AsChangeset};

use crate::db::schema::{users,profiles};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Associations, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profiles)]
pub struct Profile {
    #[serde(default)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub nickname: String,
    pub bio: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub avatar: String, 
    pub phone: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
