use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use ulid::Ulid;

use crate::{error::ClipError, models::*};


#[allow(non_snake_case)]

impl super::DatabaseWrapper {

    pub async fn user_exists(&self, target_username: String) -> Result<bool, ClipError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.get().await?;
        let res: Result<usize, diesel::result::Error> = users.filter(username.eq(target_username)).execute(&mut conn).await;
        match res {
            Ok(1) => return Err(ClipError::UserExists),
            Ok(0) => return Ok(true),
            Err(e) => return Err(ClipError::DieselError(e)),
            _ => return Err(ClipError::UnknownError),
        };
    }


    pub async fn register_user(&self, new_username: String, new_password: String, new_email: String) -> Result<bool, ClipError>{
        let password_salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(new_password.as_bytes(), &password_salt).unwrap();
        let hash = password_hash.hash.unwrap();
        let new_ulid = Ulid::new();
        let new_user = RegisterUser {
            id: new_ulid.to_string(),
            username: new_username,
            password: hash.as_bytes().to_vec(),
            salt: password_salt.as_str().as_bytes().to_vec(),
            email: new_email,
        };

        
        use crate::schema::users;

        let new_user_clone = new_user.clone();

        let mut conn = self.db.get().await?;

        let res: Result<usize, diesel::result::Error> = diesel::insert_into(users::table)
            .values(new_user_clone)
            .execute(&mut conn)
            .await;

        match res {
            Err(e) => {
                return Err(ClipError::DieselError(e))},
            Ok(_) => return Ok(true),
        };

    }
}