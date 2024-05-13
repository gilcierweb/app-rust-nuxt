use actix_web::{HttpRequest, HttpResponse, web};
use crate::db::database::Database;
use crate::models::user::User;
use crate::repositories::base_repository::BaseRepository;
use crate::repositories::users_repository::UserRepository;

pub struct PasswordsController;

impl PasswordsController {
    
// // get /users/password/new
// #[get("/users/password/new")]
// pub async fn new(){""}

// // get |  /users/password/edit
// #[get("/users/password/edit")]
// pub async fn edit(){""}

    // // post /users/password
// #[post("/users/password")]
    pub async fn create(db: web::Data<Database>, new_user: web::Json<User>, request: Option<HttpRequest>) -> HttpResponse {
        let user = UserRepository::new(db, request).create(&mut new_user.into_inner());

        match user {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

// // patch | put /users/password
// #[routes]
// #[patch("/users/password")]
// #[put("/users/password")]
// pub async fn update(){""}
}