use chrono::prelude::*;
use std::fmt::Error;

use actix_web::{web};
use diesel::{QueryDsl, RunQueryDsl};
use crate::controllers::todos_controller::Database;

use crate::db::schema::todos::dsl::todos;
use crate::models::todo::Todo;

impl Todo {
    pub fn all(db: web::Data<Database>) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error loading all todos")
    }
    pub fn find(db: web::Data<Database>, id: &str) -> Option<Todo> {
        let todo = todos
            .find(id)
            .get_result::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }
    pub fn create(db: web::Data<Database>, todo: Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..todo
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }
    pub fn update(db: web::Data<Database>, id: &str, mut todo: Todo) -> Option<Todo> {
        todo.updated_at = Some(Utc::now().naive_utc());
        let todo = diesel::update(todos.find(id))
            .set(&todo)
            .get_result::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error updating todo by id");
        Some(todo)
    }
    pub fn delete(db: web::Data<Database>, id: &str) -> Option<usize> {
        let count = diesel::delete(todos.find(id))
            .execute(&mut db.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }
}