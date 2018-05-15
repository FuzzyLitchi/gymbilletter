use rocket::http::{Cookie, Cookies, Status};
use rocket::{Outcome, Request};
use rocket::request::FromRequest;

use database::DbConn;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
    salt_hash: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, (Status, ()), ()> {
        //Get key from cookie
        let cookie: Cookie = match request.guard::<Cookies>() {
            Outcome::Success(mut cookies) => {
                match cookies.get_private("uuid") {
                    Some(s) => s,
                    _ => return Outcome::Forward(()),
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
                .first(&*conn) {
                Ok(user) => Outcome::Success(user),
                Err(_) => Outcome::Failure((Status::BadRequest, ())),
            }
        } else {
            Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}
