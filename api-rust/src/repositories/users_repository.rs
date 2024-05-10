use std::fmt::Error;
use actix_web::web;

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::users::dsl::*;
use crate::models::user::User;
use crate::repositories::base_repository::BaseRepository;

pub struct UserRepository {
    connection: web::Data<Database>,
}

impl BaseRepository<User> for UserRepository {

    fn new(connection: web::Data<Database>) -> Self {
        Self { connection }
    }

    fn all(&self) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = users.load::<User>(&mut conn).expect("Error loading all users");
        Ok(items)
    }


    fn find(&self, user_id: &Uuid) -> Option<User> {
        let user = users.find(user_id)
            .get_result::<User>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading User by id");
        Some(user)
    }

    fn create(&mut self, entity: &mut User) -> Result<User, Error> {
        let user = User {        
            ..entity.to_owned()
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new User");
        Ok(user)
    }

    fn update(&mut self, user_id: &Uuid, entity: &mut User) -> Option<User> {
        let connection = &mut self.connection.pool.get().unwrap();
        let user = diesel::update(users.find(user_id))
            .set(entity.to_owned())
            .get_result::<User>(connection)
            .expect("Error updating User by id");
        Some(user)
    }

    fn delete(&mut self, user_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(users.find(user_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting User by id");
        Some(count)
    }
}