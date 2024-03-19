use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, get, put, delete, HttpResponse};
use crate::{models::post::Post, repository::database::Database};


#[post("/posts")]
pub async fn create_post(db: Data<Database>, new_post: Json<Post>) -> HttpResponse {
    let post = db.create_post(new_post.into_inner());
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/posts")]
pub async fn get_posts(db: web::Data<Database>) -> HttpResponse {
    let posts = db.get_posts();
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.get_post_by_id(&id);
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/posts/{id}")]
pub async fn update_post_by_id(db: web::Data<Database>, id: web::Path<String>, updated_post: web::Json<Post>) -> HttpResponse {
    let post = db.update_post_by_id(&id, updated_post.into_inner());
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.delete_post_by_id(&id);
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_post)
            .service(get_posts)
            .service(get_post_by_id)
            .service(update_post_by_id)
            .service(delete_post_by_id)
    );
}
