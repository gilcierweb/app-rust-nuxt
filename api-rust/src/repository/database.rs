use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

use crate::models::post::Post;

pub struct Database {
    pub posts: Arc<Mutex<Vec<Post>>>,
}

impl Database {
    pub fn new() -> Self {
        let posts = Arc::new(Mutex::new(vec![]));
        Database { posts }
    }

    pub fn create_post(&self, post: Post) -> Result<Post, Error> {
        let mut posts = self.posts.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let status = false;
        let post = Post {
            id: Some(id),
            status: Some(status),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..post
        };
        posts.push(post.clone());
        Ok(post)
    }

    pub fn get_posts(&self) -> Vec<Post> {
        let posts = self.posts.lock().unwrap();
        posts.clone()
    }

    pub fn get_post_by_id(&self, id: &str) -> Option<Post> {
        let posts = self.posts.lock().unwrap();
        posts.iter().find(|post| post.id == Some(id.to_string())).cloned()
    }

    pub fn update_post_by_id(&self, id: &str, post: Post) -> Option<Post> {
        let mut posts = self.posts.lock().unwrap();
        let updated_at = Utc::now();
        let post = Post {
            id: Some(id.to_string()),
            updated_at: Some(updated_at),
            ..post
        };
        let index = posts.iter().position(|post| post.id == Some(id.to_string()))?;
        posts[index] = post.clone();
        Some(post)
    }
    
    pub fn delete_post_by_id(&self, id: &str) -> Option<Post> {
        let mut posts = self.posts.lock().unwrap();
        let index = posts.iter().position(|post| post.id == Some(id.to_string()))?;
        Some(posts.remove(index))
    }
    
}