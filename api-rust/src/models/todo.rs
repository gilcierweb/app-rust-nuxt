use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use uuid::Uuid;

use crate::db::schema::todos;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = todos)]
pub struct Todo {
    #[serde(default)]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    #[serde(default)]  
    pub title: String,
    pub description: Option<String>,
}
