

use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use diesel_async::RunQueryDsl;
use ulid::Ulid;

use crate::{error::ClipError, models::*};


#[allow(non_snake_case)]

impl super::DatabaseWrapper {

    pub async fn user_exists(&self, target_username: String) -> Result<String, ClipError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.get().await?;
        let res: Result<usize, diesel::result::Error> = FilterDsl::filter(users, username.eq(target_username)).execute(&mut conn).await;
        dbg!(&res);
        match res {
            Ok(1) => return Err(ClipError::UserExists),
            Ok(0) => return Ok("moi".to_string()),
            Err(e) => return Err(ClipError::DieselError(e)),
            _ => return Err(ClipError::UnknownError),
        };
    }

    pub async fn verify_user_password(&self, target_username: &str, target_password: &str) -> Result<Option<User>, ClipError> {
        use crate::schema::users::dsl::*;
        let mut conn = self.db.get().await?;

        let res: Result<User, diesel::result::Error> = FilterDsl::filter(users, username.eq(target_username)).first::<User>(&mut conn).await;
        let argon2 = Argon2::default();
        let user: User = res.unwrap();
        let password_hash = user.password.clone();
        let password_salt: Vec<u8> = user.salt.clone();
        let Ok(salt_string) = SaltString::from_b64(std::str::from_utf8(&password_salt).expect("bug: impossible"))
        else {
            return Ok(None);
        };

        let calculated_hash = argon2.hash_password(target_password.as_bytes(), &salt_string).unwrap();

        if calculated_hash.hash.expect("Bug: Impossible").as_bytes() == password_hash {
            Ok(Some(user))
        }
        else {
            Err(ClipError::InvalidCredentials)
        }
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