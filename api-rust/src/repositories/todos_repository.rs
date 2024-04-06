use chrono::prelude::*;
use std::fmt::Error;

use actix_web::{web};
use actix_web::web::Data;
use diesel::{QueryDsl, RunQueryDsl};
use crate::db::database::Database;

use crate::db::schema::todos::dsl::todos;
use crate::models::todo::Todo;
use crate::repositories::base_repository::BaseRepository;

pub struct TodoRepository {
    connection: web::Data<Database>,
}

impl BaseRepository<Todo> for TodoRepository {
    fn new(connection: Data<Database>) -> Self {
        Self { connection }
    }

    fn all(&self) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    fn find(&self, id: &str) -> Option<Todo> {
        let todo = todos
            .find(id)
            .get_result::<Todo>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }

    fn create(&mut self, entity: &mut Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..entity.to_owned()
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    fn update(&mut self, id: &str, todo: &mut Todo) -> Option<Todo> {
        todo.updated_at = Some(Utc::now().naive_utc());
        let todo = diesel::update(todos.find(id))
            .set(todo.to_owned())
            .get_result::<Todo>(&mut self.connection.pool.get().unwrap())
            .expect("Error updating todo by id");
        Some(todo)
    }

    fn delete(&mut self, id: &str) -> Option<usize> {
        let count = diesel::delete(todos.find(id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }
}