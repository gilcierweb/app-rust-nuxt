use chrono::prelude::*;
use std::fmt::Error;
use actix_web::web;

use diesel::{QueryDsl, RunQueryDsl};

use crate::db::database::Database;
use crate::db::schema::posts::dsl::*;
use crate::models::post::Post;
use crate::repositories::base_repository::BaseRepository;

pub struct PostRepository {
    connection: web::Data<Database>,
}

impl BaseRepository<Post> for PostRepository {
    fn new(connection: web::Data<Database>) -> Self {
        Self { connection }
    }

    fn all(&self) -> Vec<Post> {
        posts
            .load::<Post>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading all posts")
        // Ok(posts)
    }

    fn find(&self, post_id: &str) -> Option<Post> {
        let post = posts
            .find(post_id)
            .get_result::<Post>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading post by id");
        Some(post)
    }

    fn create(&mut self, entity: &mut Post) -> Result<Post, Error> {
        let post = Post {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..entity.to_owned()
        };
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new post");
        Ok(post)
    }

    fn update(&mut self, post_id: &str, entity: &mut Post) -> Option<Post> {
        entity.updated_at = Some(Utc::now().naive_utc());
        // println!(&entity.id);
        let post = diesel::update(posts.find(post_id))
            .set(entity.to_owned())
            .get_result::<Post>(&mut self.connection.pool.get().unwrap())
            .expect("Error updating post by id");
        Some(post)
    }

    fn delete(&mut self, post_id: &str) -> Option<usize> {
        let count = diesel::delete(posts.find(post_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting post by id");
        Some(count)
    }
}