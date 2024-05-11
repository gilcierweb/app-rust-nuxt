use std::fmt::Error;
use actix_web::{web, HttpRequest};
use uuid::Uuid;

use crate::db::database::Database;

pub trait BaseRepository<TEntity> {
    /// create a new repository with the connection
    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self;
    fn all(&self) -> Result<Vec<TEntity>, diesel::result::Error>;
    fn find(&self, id: &Uuid) -> Option<TEntity>;
    fn create(&mut self,  entity: &mut TEntity) -> Result<TEntity, Error>;
    fn update(&mut self, id: &Uuid, entity: &mut TEntity) -> Option<TEntity>;
    fn delete(&mut self, id: &Uuid) -> Option<usize>;
}

