use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use uuid::Uuid;
use crate::db::database::Database;
use crate::{models::user::Profile, repositories::base_repository::BaseRepository,
            repositories::profiles_repository::ProfileRepository};

#[get("/profiles")]
pub async fn get_profiles(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let result = ProfileRepository::new(db).all();
    match result {
        Ok(profiles) => {
            let response = serde_json::to_string(&profiles).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[post("/profiles")]
pub async fn create_user(db: web::Data<Database>, new_user: web::Json<Profile>) -> HttpResponse {
    let user = ProfileRepository::new(db).create(&mut new_user.into_inner());

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles/{id}")]
pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let user = ProfileRepository::new(db).find(&id);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}

#[put("/profiles/{id}")]
pub async fn update_user_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_user: web::Json<Profile>,
) -> HttpResponse {
    let user = ProfileRepository::new(db).update(&id, &mut updated_user.into_inner());

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}

#[delete("/profiles/{id}")]
pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let user = ProfileRepository::new(db).delete(&id);

    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}