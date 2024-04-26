use chrono::prelude::*;
use diesel::prelude::*;
use fakeit::words;

#[path = "../models/mod.rs"]
mod models;
#[path = "../repositories/mod.rs"]
mod repositories;
#[path = "../db/mod.rs"]
pub mod db;

use crate::db::schema::posts::dsl::*;
use crate::db::schema::todos::dsl::*;
use crate::{models::post::Post, models::todo::Todo, db::database::Database};

pub fn post_create_seed(db: &mut Database) {
    println!("Seed Posts");
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
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

pub fn todo_create_seed(db: &mut Database) {
    println!("Seed Todos");
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
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

fn main() {
    let mut api_db = Database::new();
    println!("rust db:seed start");
    post_create_seed(&mut api_db);
    todo_create_seed(&mut api_db);
    println!("rust db:seed end");
}
