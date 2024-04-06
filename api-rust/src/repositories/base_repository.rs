use std::fmt::Error;
use crate::controllers::posts_controller::Post;
use crate::db::database::Database;

pub trait BaseRepository<TEntity> {
    /// create a new repository with the connection
    fn new(connection: Database) -> Self;
    fn all(&self) -> Vec<TEntity>;
    fn find(&self, id: &str) -> Option<TEntity>;
    fn create(&mut self,  entity: &mut TEntity) -> Result<TEntity, Error>;
    fn update(&mut self, id: &str, entity: &mut TEntity) -> Option<TEntity>;
    fn delete(&mut self, id: &str) -> Option<usize>;
}

