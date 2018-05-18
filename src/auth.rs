use rocket::http::{Cookie, Cookies, Status};
use rocket::{Outcome, Request};
use rocket::request::FromRequest;

use database::DbConn;
use uuid::Uuid;
use schema::users;

#[derive(Queryable)]
pub struct UserQuery {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
    salt_hash: String,
}

impl UserQuery {
    pub fn downgrade(self) -> User {
        User {
            username: self.username,
            is_admin: self.is_admin,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub is_admin: bool
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, (Status, ()), ()> {
        //Get key from cookie
        let cookie: Cookie = match request.guard::<Cookies>() {
            Outcome::Success(mut cookies) => {
                match cookies.get_private("uuid") {
                    Some(s) => s,
                    None => return Outcome::Forward(()),
                }
            },
            _ => return Outcome::Forward(()),
        };


        //Validate the uuid
        let uuid: Uuid = match Uuid::parse_str(cookie.value()) {
            Ok(ok) => ok,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
        };

        //Check database
        if let Outcome::Success(conn) = request.guard::<DbConn>() {
            use schema::users::dsl::users;
            use diesel::prelude::*;

            return match users.find(uuid)
                .first::<UserQuery>(&*conn) {
                Ok(user) => Outcome::Success(user.downgrade()),
                // In theory, this shouldn't happen.
                // The only way it can is if the server sets a private cookie to be a valid uuid,
                // then deletes the equivalent user. Or someone has the ability to set custom
                // private cookies. Which should be impossible.
                Err(_) => Outcome::Failure((Status::BadRequest, ())),
            }
        } else {
            Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}

#[derive(Insertable, FromForm)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub is_admin: bool,
    salt_hash: String,
}
