use crate::db::database::Database;
use crate::{
    models::user::User, repositories::base_repository::BaseRepository,
    repositories::users_repository::UserRepository,
};

use actix_web::{ web, HttpRequest, HttpResponse};
use uuid::Uuid;

pub struct RegistrationsController;

impl RegistrationsController {
    // get /users/cancel
    // #[get("/users/cancel")]
    pub async fn cancel() {}

    // get /users/sign_up
    // #[get("/users/sign_up")]
    pub async fn new() {}

    // get |  /users/edit
    // #[get("/users/edit")]
    pub async fn edit() {}

    // post /users
    // #[post("/users")]
    pub async fn create(db: web::Data<Database>, new_user: web::Json<User>, request: Option<HttpRequest>) -> HttpResponse {
        // if let Some(val) = request.peer_addr() {
        //     println!("Address {:?}", val.ip());
        // };
        let user = UserRepository::new(db, request).create(&mut new_user.into_inner());

        match user {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    // patch | put /users
    // #[routes]
    // #[patch("/users")]
    // #[put("/users")]
    pub async fn update(db: web::Data<Database>, id: web::Path<Uuid>, updated_user: web::Json<User>) -> HttpResponse {
        let user = UserRepository::new(db, None).update(&id, &mut updated_user.into_inner());

        match user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().body("User not found"),
        }
    }

    // delete /users
    // #[delete("/users")]
    pub async fn destroy(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
        let user = UserRepository::new(db, None).delete(&id);

        match user {
            Some(_) => HttpResponse::Ok().finish(),
            None => HttpResponse::NotFound().body("User not found"),
        }
    }
}
