// use crate::db::database::Database;
// use crate::{
//     models::user::User, repositories::base_repository::BaseRepository,
//     repositories::users_repository::UserRepository,
// };

// use actix_web::{delete, delete, get, patch, post, put, web, HttpResponse};

// pub struct SessionsController;

// impl SessionsController {
//     // get /users/cancel
//     // #[get("/users/cancel")]
//     pub async fn cancel() {}

//     // get /users/sign_up
//     // #[get("/users/sign_up")]
//     pub async fn new() {}

//     // get |  /users/edit
//     // #[get("/users/edit")]
//     pub async fn edit() {}

//     // post /users
//     // #[post("/users")]
//     pub async fn create(db: web::Data<Database>, new_user: web::Json<User>) -> HttpResponse {
//         let user = UserRepository::new(db).create(&mut new_user.into_inner());

//         match user {
//             Ok(user) => HttpResponse::Ok().json(user),
//             Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//         }
//     }

//     // patch | put /users
//     // #[routes]
//     // #[patch("/users")]
//     // #[put("/users")]
//     pub async fn update() {}

//     // delete /users
//     // #[delete("/users")]
//     pub async fn destroy() {}

// }
