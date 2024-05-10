use std::fmt::Error;
use actix_web::web;

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::posts::dsl::*;
use crate::models::post::{Post};
use crate::repositories::base_repository::BaseRepository;

pub struct PostRepository {
    connection: web::Data<Database>,
}

impl BaseRepository<Post> for PostRepository {
    fn new(connection: web::Data<Database>) -> Self {
        Self { connection }
    }

    fn all(&self) -> Result<Vec<Post>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = posts.load::<Post>(&mut conn).expect("Error loading all posts");
        Ok(items)
    }

    fn find(&self, post_id: &Uuid) -> Option<Post> {
        let post = posts.find(post_id)
            .get_result::<Post>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading post by id");
        Some(post)
    }

    fn create(&mut self, entity: &mut Post) -> Result<Post, Error> {
        let post = Post {           
            ..entity.to_owned()
        };
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new post");
        Ok(post)
    }

    fn update(&mut self, post_id: &Uuid, entity: &mut Post) -> Option<Post> {
        // let post = diesel::update(posts.find(post_id))
        //     .set(entity.to_owned())
        //     .get_result::<Post>(&mut self.connection.pool.get().unwrap())
        //     .expect("Error updating post by id");
        let post = diesel::update(posts.find(post_id))
            .set(entity.to_owned())          
            .get_result::<Post>(&mut self.connection.pool.get().unwrap())
            .expect("Error updating post by id");
        Some(post)
    }

    fn delete(&mut self, post_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(posts.find(post_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting post by id");
        Some(count)
    }
}