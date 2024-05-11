use actix_web::{delete, Error, get, HttpRequest, HttpResponse, post, put, web};
use uuid::Uuid;
use crate::db::database::Database;
use crate::{models::user::User, repositories::base_repository::BaseRepository,
            repositories::users_repository::UserRepository};

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let result = UserRepository::new(db, None).all();
    match result {
        Ok(users) => {
            let response = serde_json::to_string(&users).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[post("/users")]
pub async fn create_user(db: web::Data<Database>, new_user: web::Json<User>, request: Option<HttpRequest>) -> HttpResponse {
    let user = UserRepository::new(db,request).create(&mut new_user.into_inner());

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let user = UserRepository::new(db,None).find(&id);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/users/{id}")]
pub async fn update_user_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_user: web::Json<User>,
) -> HttpResponse {
    let user = UserRepository::new(db,None).update(&id, &mut updated_user.into_inner());

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let user = UserRepository::new(db,None).delete(&id);

    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("User not found"),
    }
}