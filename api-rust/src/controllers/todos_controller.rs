use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use uuid::Uuid;
use crate::db::database::Database;
use crate::{models::todo::Todo, repositories::base_repository::BaseRepository,
            repositories::todos_repository::TodoRepository};

#[get("/todos")]
pub async fn get_todos(db: web::Data<Database>) -> Result<HttpResponse, Error>  {
    let result = TodoRepository::new(db).all();
    match result {
        Ok(todos) => {
            let response = serde_json::to_string(&todos).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[post("/todos")]
pub async fn create_todo(db: web::Data<Database>, new_todo: web::Json<Todo>) -> HttpResponse {
    let todo = TodoRepository::new(db).create(&mut new_todo.into_inner());

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let todo = TodoRepository::new(db).find(&id);

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = TodoRepository::new(db).update(&id, &mut updated_todo.into_inner());

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let todo = TodoRepository::new(db).delete(&id);

    match todo {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}