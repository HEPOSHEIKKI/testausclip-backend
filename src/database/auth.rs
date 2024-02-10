use super::establish_connection;

use diesel::{prelude::*, query_dsl::methods::FilterDsl};

use crate::models::UserName;


pub async fn user_exists(target_username: String) -> Result<bool, ()> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    dbg!(&target_username);
    let operation =
        FilterDsl::filter(users, username.eq(target_username))
            .select(UserName::as_select())
            .first(connection)
            .optional();

    match operation {
        Ok(found) => {
            match found {
                Some(_) => return Ok(true),
                None => return Ok(false)
            }
        }
        Err(_) => return Err(())
    };
}