pub use crate::{models::todo::Todo, repository::database::Database, controllers::posts_controller,controllers::todos_controller};
use actix_web::{web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(todos_controller::create_todo)
            .service(todos_controller::get_todo_by_id)
            .service(todos_controller::get_todos)
            .service(todos_controller::delete_todo_by_id)
            .service(todos_controller::update_todo_by_id)
            .service(posts_controller::create_post)
            .service(posts_controller::get_post_by_id)
            .service(posts_controller::get_posts)
            .service(posts_controller::delete_post_by_id)
            .service(posts_controller::update_post_by_id),
    );
}
