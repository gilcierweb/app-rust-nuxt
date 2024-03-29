use chrono::prelude::*;
use diesel::prelude::*;
use fakeit::words;

#[path = "../models/mod.rs"]
mod models;
#[path = "../repository/mod.rs"]
mod repository;

use crate::repository::schema::posts::dsl::*;
use crate::repository::schema::todos::dsl::*;
use crate::{models::post::Post, models::todo::Todo, repository::database::Database};

fn post_create_seed() {
    let api_db = Database::new();

    for index in 1..12 {
        println!("{}", index);
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let post = Post {
            id: uuid::Uuid::new_v4().to_string(),
            title: words::sentence(5),
            content: Some(paragraph),
            status: Some(true),

            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        };

        let _ = diesel::insert_into(posts)
            .values(&post)
            .execute(&mut api_db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

fn todo_create_seed() {
    let api_db = Database::new();

    for index in 1..12 {
        println!("{}", index);
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            title: words::sentence(5),
            description: Some(paragraph),

            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        };

        let _ = diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut api_db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

fn main() {
    println!("rust db:seed");
    post_create_seed();
    todo_create_seed();
    println!("rust db:seed end");
}
