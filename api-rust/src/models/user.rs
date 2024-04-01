// use serde::{Deserialize, Serialize};
// use diesel::{Queryable, Insertable, AsChangeset};

// #[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
// #[diesel(table_name = crate::repository::schema::users)]
// pub struct User {
//     #[serde(default)]
//     pub id: String,
//     pub email: String,
//     pub password: String,
//     pub hash: String,
//     pub token: String,
//     pub created_at: Option<chrono::NaiveDateTime>,
//     pub updated_at: Option<chrono::NaiveDateTime>,
// }
