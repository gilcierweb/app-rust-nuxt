use actix_web::{delete, get, post, put, web, HttpResponse};
pub use crate::{models::post::Post, repositories::database::Database};
// use crate::api::api::{Database, Post};

#[get("/posts")]
pub async fn get_posts(db: web::Data<Database>) -> HttpResponse {
    let posts = db.get_posts();
    HttpResponse::Ok().json(posts)
}

#[post("/posts")]
pub async fn create_post(db: web::Data<Database>, new_post: web::Json<Post>) -> HttpResponse {
    let post = db.create_post(new_post.into_inner());
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/posts/{id}")]
pub async fn get_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.get_post_by_id(&id);

    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[put("/posts/{id}")]
pub async fn update_post_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_post: web::Json<Post>,
) -> HttpResponse {
    let post = db.update_post_by_id(&id, updated_post.into_inner());
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.delete_post_by_id(&id);
    match post {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}
