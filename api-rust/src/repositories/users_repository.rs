use actix_web::{web, HttpRequest};
use chrono::Utc;

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;
use crate::auth::password::password_hash;

use crate::db::database::Database;
use crate::db::schema::users::dsl::*;
use crate::models::user::{NewUser, User};
use crate::repositories::base_repository::BaseRepository;


pub struct UserRepository {
    connection: web::Data<Database>,
    request: Option<HttpRequest>, // Optional HttpRequest
}

impl BaseRepository<User> for UserRepository {
    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self {
        Self {
            connection,
            request,
        }
    }

    fn all(&self) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = users
            .load::<User>(&mut conn)
            .expect("Error loading all users");
        Ok(items)
    }

    fn find(&self, user_id: &Uuid) -> Option<User> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading User by id");
        Some(user)
    }

    fn create(&mut self, entity: &mut User) -> Result<User, std::fmt::Error> {
  

        let ip = self
            .request
            .as_ref()
            .and_then(|req| req.peer_addr())
            .map(|addr| addr.to_string());

        let new_user = NewUser {
            encrypted_password: password_hash(entity.encrypted_password.clone()),
            email: entity.email.clone(),
            current_sign_in_at: Some(Utc::now().naive_utc()), // Default timestamp
            current_sign_in_ip: ip.clone(),
            last_sign_in_at: Some(Utc::now().naive_utc()), // Default timestamp
            last_sign_in_ip: ip.clone(),
        };

        let mut conn = self.connection.pool.get().unwrap();
        let inserted_user = diesel::insert_into(users)
            .values(&new_user)  
            .get_result::<User>(&mut conn)
            .expect("Error creating new User");

        Ok(inserted_user)
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
