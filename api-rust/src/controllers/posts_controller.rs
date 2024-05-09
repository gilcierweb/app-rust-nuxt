use actix_web::{delete, get, post, put, web, HttpResponse,Error};

use uuid::Uuid;
pub use crate::{models::post::Post, repositories::base_repository::BaseRepository,
                repositories::posts_repository::PostRepository};

use crate::db::database::Database;

#[get("/posts")]
pub async fn get_posts(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let result = PostRepository::new(db).all();
    match result {
        Ok(posts) => {
            let response = serde_json::to_string(&posts).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[post("/posts")]
pub async fn create_post(db: web::Data<Database>, new_post: web::Json<Post>) -> HttpResponse {
    let post = PostRepository::new(db).create(&mut new_post.into_inner());

    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/posts/{id}")]
pub async fn get_post_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let post = PostRepository::new(db).find(&id);

    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[put("/posts/{id}")]
pub async fn update_post_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_post: web::Json<Post>,
) -> HttpResponse {
    let post = PostRepository::new(db).update(&id, &mut updated_post.into_inner());

    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let post = PostRepository::new(db).delete(&id);

    match post {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}
