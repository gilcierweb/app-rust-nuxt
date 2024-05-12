pub use crate::{controllers::posts_controller, controllers::todos_controller, controllers::users_controller,
                controllers::profiles_controller, controllers::users::registrations_controller};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(todos_controller::create_todo)
            .service(todos_controller::get_todo_by_id)
            .service(todos_controller::get_todos)
            .service(todos_controller::update_todo_by_id)
            .service(todos_controller::delete_todo_by_id)
            .service(posts_controller::create_post)
            .service(posts_controller::get_post_by_id)
            .service(posts_controller::get_posts)
            .service(posts_controller::update_post_by_id)
            .service(posts_controller::delete_post_by_id)
            .service(web::scope("/v1")
                .service(users_controller::create_user)
                .service(users_controller::get_user_by_id)
                .service(users_controller::get_users)
                .service(users_controller::update_user_by_id)
                .service(users_controller::delete_user_by_id)
                .service(profiles_controller::create_profile)
                .service(profiles_controller::get_profile_by_id)
                .service(profiles_controller::get_profiles)
                .service(profiles_controller::update_profile_by_id)
                .service(profiles_controller::delete_profile_by_id)
            )
            .service(web::scope("/v2")
                .service(web::resource("/users").route(web::post().to(registrations_controller::RegistrationsController::create)))
                .service(users_controller::update_user_by_id)
                .service(users_controller::delete_user_by_id)
            )
        ,
    );
}
