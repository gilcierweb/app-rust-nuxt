use std::fmt::Error;
use actix_web::HttpRequest;

use actix_web::web::Data;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::todos::dsl::todos;
use crate::models::todo::Todo;
use crate::repositories::base_repository::BaseRepository;

pub struct TodoRepository {
    connection: Data<Database>,
    request: Option<HttpRequest> // Optional HttpRequest
}

impl BaseRepository<Todo> for TodoRepository {
    fn new(connection: Data<Database>, request: Option<HttpRequest>) -> Self {
        Self { connection, request }
    }

    fn all(&self) -> Result<Vec<Todo>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = todos.load::<Todo>(&mut conn).expect("Error loading all todos");
        Ok(items)
    }

    fn find(&self, id: &Uuid) -> Option<Todo> {
        let todo = todos
            .find(id)
            .get_result::<Todo>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }

    fn create(&mut self, entity: &mut Todo) -> Result<Todo, Error> {
        let todo = Todo {           
            ..entity.to_owned()
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    fn update(&mut self, id: &Uuid, todo: &mut Todo) -> Option<Todo> {
 
        let todo = diesel::update(todos.find(id))
            .set(todo.to_owned())
            .get_result::<Todo>(&mut self.connection.pool.get().unwrap())
            .expect("Error updating todo by id");
        Some(todo)
    }

    fn delete(&mut self, id: &Uuid) -> Option<usize> {
        let count = diesel::delete(todos.find(id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }
}