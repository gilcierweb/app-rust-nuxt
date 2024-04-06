use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder, Result, middleware};
use serde::Serialize;
use std::env;
use std::string::String;

// Import modules is required for use crate::mymod::
mod routes;
mod models;
mod repositories;
mod db;
mod auth;
mod config;
mod controllers;

#[derive(Serialize)]
pub struct Response {
    // pub status: String,
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_db = db::database::Database::new();
    let app_data = web::Data::new(api_db);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let host = env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse::<String>()
        .expect("HOST must be a number");

    let frontend_url: String = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .parse::<String>()
        .expect("FRONTEND_URL is not set");

    println!("Running in http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&frontend_url)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .configure(routes::router::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::Compress::default())
    })
        .bind((host, port))?
        .run()
        .await
}
