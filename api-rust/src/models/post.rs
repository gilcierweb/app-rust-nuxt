use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, AsChangeset, Selectable};
use uuid::Uuid;

use crate::db::schema::posts;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
    #[serde(default)]
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub status: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable,Insertable,)]
#[diesel(table_name = posts)]
pub struct NewPost {   
    pub title: String,
    pub content: Option<String>,
    pub status: Option<bool>
}