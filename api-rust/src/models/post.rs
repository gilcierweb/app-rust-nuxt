use serde::{Deserialize, Serialize};

use diesel::{Queryable, Insertable, AsChangeset};


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::posts)]
pub struct Post {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub status: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}