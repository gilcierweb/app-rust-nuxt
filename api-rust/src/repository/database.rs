use chrono::prelude::*;
use std::fmt::Error;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::todo::Todo;
use crate::db::schema::todos::dsl::*;

use crate::models::post::Post;
use crate::db::schema::posts::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
   pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..todo
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    pub fn get_todo_by_id(&self, todo_id: &str) -> Option<Todo> {
        let todo = todos
            .find(todo_id)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }

    pub fn delete_todo_by_id(&self, todo_id: &str) -> Option<usize> {
        let count = diesel::delete(todos.find(todo_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }

    pub fn update_todo_by_id(&self, todo_id: &str, mut todo: Todo) -> Option<Todo> {
        todo.updated_at = Some(Utc::now().naive_utc());
        let todo = diesel::update(todos.find(todo_id))
            .set(&todo)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error updating todo by id");
        Some(todo)
    }

    // Posts

    pub fn get_posts(&self) -> Vec<Post> {
        posts
            .load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all posts")
    }

    pub fn create_post(&self, post: Post) -> Result<Post, Error> {
        let post = Post {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..post
        };
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new post");
        Ok(post)
    }

    pub fn get_post_by_id(&self, post_id: &str) -> Option<Post> {
        let post = posts
            .find(post_id)
            .get_result::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading post by id");
        Some(post)
    }

    pub fn delete_post_by_id(&self, post_id: &str) -> Option<usize> {
        let count = diesel::delete(posts.find(post_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting post by id");
        Some(count)
    }

    pub fn update_post_by_id(&self, post_id: &str, mut post: Post) -> Option<Post> {
        post.updated_at = Some(Utc::now().naive_utc());
        let post = diesel::update(posts.find(post_id))
            .set(&post)
            .get_result::<Post>(&mut self.pool.get().unwrap())
            .expect("Error updating post by id");
        Some(post)
    }
}
