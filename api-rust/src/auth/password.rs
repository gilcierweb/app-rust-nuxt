pub async fn hash() {}

pub async fn verify_password() {}

pub fn password_hash(password: String) -> String {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.clone().as_bytes(), &salt)
        .expect("Unable to hash password.")
        .to_string();

    password_hash
}