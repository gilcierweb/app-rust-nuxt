use chrono::prelude::*;
use std::fmt::Error;

use actix_web::{web};
use diesel::{QueryDsl, RunQueryDsl};
use crate::db::database::Database;

use crate::models::post::Post;
use crate::db::schema::posts::dsl::*;
use crate::repositories::base_repository::BaseRepository;

struct PostRepository;
impl BaseRepository<Post> for PostRepository{
    fn all(&self) -> Result<Vec<Post>, String> {
        posts
            .load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all posts")
    }

    fn find(&self, id: &str) -> Result<Option<Post>, String> {
        todo!()
    }

    fn create(&mut self, entity: &Post) -> Result<(), String> {
        todo!()
    }

    fn update(&mut self, entity: &Post) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        todo!()
    }
}

// impl Post {
//     pub fn all(db: web::Data<Database>) -> Vec<Post> {
//         posts
//             .load::<Post>(&mut db.pool.get().unwrap())
//             .expect("Error loading all posts")
//     }
//     pub fn find(db: web::Data<Database>, id: &str) -> Option<Post> {
//         let post = posts
//             .find(id)
//             .get_result::<Post>(&mut db.pool.get().unwrap())
//             .expect("Error loading todo by id");
//         Some(post)
//     }
//     pub fn create(db: web::Data<Database>, post: Post) -> Result<Post, Error> {
//         let post = Post {
//             id: uuid::Uuid::new_v4().to_string(),
//             created_at: Some(Utc::now().naive_utc()),
//             updated_at: Some(Utc::now().naive_utc()),
//             ..post
//         };
//         diesel::insert_into(posts)
//             .values(&post)
//             .execute(&mut db.pool.get().unwrap())
//             .expect("Error creating new post");
//         Ok(post)
//     }
//     pub fn update(db: web::Data<Database>, id: &str, mut post: Post) -> Option<Post> {
//         post.updated_at = Some(Utc::now().naive_utc());
//         let post = diesel::update(posts.find(id))
//             .set(&post)
//             .get_result::<Post>(&mut db.pool.get().unwrap())
//             .expect("Error updating post by id");
//         Some(post)
//     }
//     pub fn delete(db: web::Data<Database>, id: &str) -> Option<usize> {
//         let count = diesel::delete(posts.find(id))
//             .execute(&mut db.pool.get().unwrap())
//             .expect("Error deleting post by id");
//         Some(count)
//     }
// }