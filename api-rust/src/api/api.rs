pub use crate::{models::post::Post, models::todo::Todo, repository::database::Database};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/todos")]
pub async fn create_todo(db: web::Data<Database>, new_todo: web::Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.get_todo_by_id(&id);

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[get("/todos")]
pub async fn get_todos(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo_by_id(&id);
    match todo {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

// posts

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

#[get("/posts")]
pub async fn get_posts(db: web::Data<Database>) -> HttpResponse {
    let posts = db.get_posts();
    HttpResponse::Ok().json(posts)
}

#[delete("/posts/{id}")]
pub async fn delete_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.delete_post_by_id(&id);
    match post {
        Some(_) => HttpResponse::Ok().finish(),
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_todo)
            .service(get_todo_by_id)
            .service(get_todos)
            .service(delete_todo_by_id)
            .service(update_todo_by_id)
            .service(create_post)
            .service(get_post_by_id)
            .service(get_posts)
            .service(delete_post_by_id)
            .service(update_post_by_id),
    );
}
