use actix_web::{HttpRequest, HttpResponse, web};
use crate::db::database::Database;
use crate::models::user::User;
use crate::repositories::base_repository::BaseRepository;
use crate::repositories::users_repository::UserRepository;

pub struct SessionsController;

impl SessionsController {
// get /users/sign_in
// #[get("/users/sign_in")]
// pub async fn new(){}

    // post /users/sign_in
    // #[post("/users/sign_in")]
    pub async fn create(db: web::Data<Database>, new_user: web::Json<User>, request: Option<HttpRequest>) -> HttpResponse {
        let user = UserRepository::new(db, request).create(&mut new_user.into_inner());

        match user {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

// delete /users/sign_out
// #[delete("/users/sign_out")]
// pub async fn destroy(){}
}