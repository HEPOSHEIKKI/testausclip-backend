use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{error::ClipError, models::*};


impl super::DatabaseWrapper {
    pub async fn user_exists(&self, target_username: String) -> Result<bool, ClipError> {
        let mut conn = self.db.get().await?;
        use crate::schema::user_identities::dsl::*;

        Ok(user_identities
            .filter(username.eq(target_username))
            .first::<UserIdentity>(&mut conn)
            .await
            .optional()?
            .is_some()
        )
    }
}


// pub async fn user_exists(target_username: String) -> Result<bool, ()> {
//     use crate::schema::users::dsl::*;
//     let connection = &mut establish_connection();
//     dbg!(&target_username);
//     let operation =
//         FilterDsl::filter(users, username.eq(target_username))
//             .select(UserName::as_select())
//             .first(connection)
//             .optional();

//     match operation {
//         Ok(found) => {
//             match found {
//                 Some(_) => return Ok(true),
//                 None => return Ok(false)
//             }
//         }
//         Err(_) => return Err(())
//     };
// }