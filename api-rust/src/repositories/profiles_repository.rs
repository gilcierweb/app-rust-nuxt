use std::fmt::Error;
use actix_web::{web, HttpRequest};

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::profiles::dsl::*;
use crate::models::profile::Profile;
use crate::repositories::base_repository::BaseRepository;

pub struct ProfileRepository {
    connection: web::Data<Database>,
    request: Option<HttpRequest> // Optional HttpRequest

}

impl BaseRepository<Profile> for ProfileRepository {

    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self {
        Self { connection, request }
    }

    fn all(&self) -> Result<Vec<Profile>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = profiles.load::<Profile>(&mut conn).expect("Error loading all profiles");
        Ok(items)
    }


    fn find(&self, profile_id: &Uuid) -> Option<Profile> {
        let profile = profiles.find(profile_id)
            .get_result::<Profile>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading Profile by id");
        Some(profile)
    }

    fn create(&mut self, entity: &mut Profile) -> Result<Profile, Error> {
        let profile = Profile {        
            ..entity.to_owned()
        };
        diesel::insert_into(profiles)
            .values(&profile)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new Profile");
        Ok(profile)
    }

    fn update(&mut self, profile_id: &Uuid, entity: &mut Profile) -> Option<Profile> {
        let connection = &mut self.connection.pool.get().unwrap();
        let profile = diesel::update(profiles.find(profile_id))
            .set(entity.to_owned())
            .get_result::<Profile>(connection)
            .expect("Error updating Profile by id");
        Some(profile)
    }

    fn delete(&mut self, profile_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(profiles.find(profile_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting Profile by id");
        Some(count)
    }
}