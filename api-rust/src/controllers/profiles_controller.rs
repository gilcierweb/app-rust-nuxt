use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use uuid::Uuid;
use crate::db::database::Database;
use crate::{models::profile::Profile, repositories::base_repository::BaseRepository,
            repositories::profiles_repository::ProfileRepository};

#[get("/profiles")]
pub async fn get_profiles(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let result = ProfileRepository::new(db, None).all();
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
pub async fn create_profile(db: web::Data<Database>, new_profile: web::Json<Profile>) -> HttpResponse {
    let profile = ProfileRepository::new(db, None).create(&mut new_profile.into_inner());

    match profile {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles/{id}")]
pub async fn get_profile_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let profile = ProfileRepository::new(db, None).find(&id);

    match profile {
        Some(profile) => HttpResponse::Ok().json(profile),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}

#[put("/profiles/{id}")]
pub async fn update_profile_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_profile: web::Json<Profile>,
) -> HttpResponse {
    let profile = ProfileRepository::new(db, None).update(&id, &mut updated_profile.into_inner());

    match profile {
        Some(profile) => HttpResponse::Ok().json(profile),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}

#[delete("/profiles/{id}")]
pub async fn delete_profile_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let profile = ProfileRepository::new(db, None).delete(&id);

    match profile {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Profile not found"),
    }
}