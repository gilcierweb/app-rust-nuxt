use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

use crate::db::schema::todos;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = todos)]
pub struct Todo {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
