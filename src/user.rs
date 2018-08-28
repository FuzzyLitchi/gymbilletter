use rocket::http::{Cookie, Cookies, Status};
use rocket::{Outcome, Request};
use rocket::request::{Form, FromRequest};

use database::DbConn;
use uuid::Uuid;
use bcrypt;
use schema::users;

use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserQuery {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
    pub hash: String,
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

#[derive(Serialize, Deserialize)]
pub struct Admin {
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Admin, (Status, ()), ()> {
        match request.guard::<User>() {
            Outcome::Success(user) => {
                if user.is_admin {
                    Outcome::Success(Admin {
                        username: user.username
                    })
                } else {
                    Outcome::Forward(())
                }
            },
            Outcome::Failure(fail) => Outcome::Failure(fail),
            Outcome::Forward(_) => Outcome::Forward(())
        }
    }
}

#[derive(FromForm)]
struct FormUser {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

impl FormUser {
    fn upgrade(&self) -> NewUser {
        NewUser {
            username: self.username.clone(),
            is_admin: self.is_admin,
            hash: bcrypt::hash(&self.password, super::DEFAULT_COST).unwrap(),
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub is_admin: bool,
    pub hash: String,
}

#[post("/new", data = "<form_user>")]
fn new(form_user: Form<FormUser>, conn: DbConn) -> String {
    use schema::users::dsl::users;
    use user::UserQuery;

    //NOTE: Check if user can be added here

    match diesel::insert_into(users)
        .values(form_user.get().upgrade())
        .get_result::<UserQuery>(&*conn) {
        Ok(user) => format!("User \"{}\" has been created, width uuid \"{}\"", user.username, user.user_id),
        Err(_) => "Error".into(),
    }
}
