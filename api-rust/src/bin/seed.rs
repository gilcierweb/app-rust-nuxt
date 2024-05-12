use chrono::{Duration, NaiveDate, TimeZone, Utc};
use diesel::prelude::*;
use fakeit::{contact, image, internet, name, password, words};
use uuid::Uuid;

#[path = "../models/mod.rs"]
mod models;
#[path = "../repositories/mod.rs"]
mod repositories;
#[path = "../db/mod.rs"]
pub mod db;

#[path = "../auth/mod.rs"]
pub mod auth;

use crate::db::schema::posts::dsl::*;
use crate::db::schema::todos::dsl::*;
use crate::{models::post::NewPost, models::todo::NewTodo, db::database::Database};
use crate::db::schema::profiles::dsl::profiles;
use crate::db::schema::profiles::user_id;
use crate::db::schema::users::dsl::*;
use crate::models::profile::NewProfile;
use crate::models::user::NewUser;

pub fn post_create_seed(db: &mut Database) {
    println!("Seed Posts");
    for index in 1..12 {
        println!("{}", index);

        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let post = NewPost {
            title: words::sentence(5),
            content: Some(paragraph),
            status: Some(true),
        };

        let _ = diesel::insert_into(posts)
            .values(&post)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

pub fn todo_create_seed(db: &mut Database) {
    println!("Seed Todos");
    for index in 1..12 {
        println!("{}", index);
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let todo = NewTodo {
            title: words::sentence(5),
            description: Some(paragraph),
        };

        let _ = diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new post");
    }
}

pub fn user_create_seed(db: &mut Database) {
    println!("Seed Users");
    for index in 1..12 {
        println!("{}", index);

        let fake_password = password::generate(true, true, true, 26); // You can use fakeit to generate a fake password if needed
        let fake_email = contact::email();
        let fake_ip = internet::ipv4_address();
        let fake_timestamp = Utc::now().naive_utc();

        let user = NewUser {
            encrypted_password: fake_password,
            email: fake_email,
            current_sign_in_at: Some(fake_timestamp),
            current_sign_in_ip: Some(fake_ip.clone()),
            last_sign_in_at: Some(fake_timestamp),
            last_sign_in_ip: Some(fake_ip.clone()),
        };

        let uid: Uuid = diesel::insert_into(users)
            .values(&user)
            .returning(crate::db::schema::users::dsl::id)
            .get_result::<Uuid>(&mut db.pool.get().unwrap())
            .expect("Error creating new user");
        println!("{}", uid);

        profile_create_seed(db, uid);
    }
}

pub fn profile_create_seed(db: &mut Database, uid: Uuid) {
    println!("Seed Profiles");

    let paragraph = words::paragraph(5, 4, 11, "\n".to_string());
    // let birth_date =  (Utc::now() - Duration::days(450));

    let profile = NewProfile {
        first_name: Some(name::first()),
        last_name: Some(name::last()),
        full_name: Some(name::full()),
        nickname: Some(internet::username()),
        bio: Some(paragraph),
        birthday: NaiveDate::from_ymd_opt(2015, 3, 14),
        avatar: Some(image::url(500, 500)),
        phone: Some(contact::phone().parse::<i64>().unwrap()),
        user_id: uid,
    };

    println!("{}", uid);

    let _ = diesel::insert_into(profiles)
        .values(&profile)
        .execute(&mut db.pool.get().unwrap())
        .expect("Error creating new post");
}

fn main() {
    let mut api_db = Database::new();
    println!("rust db:seed start");
    post_create_seed(&mut api_db);
    todo_create_seed(&mut api_db);
    user_create_seed(&mut api_db);
    println!("rust db:seed end");
}
