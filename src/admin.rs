//This file is for the admin interface
use rocket::request::Form;
use rocket_contrib::Template;

use database::DbConn;
use uuid::Uuid;
use bcrypt;
use schema::users;

use diesel;
use diesel::prelude::*;

#[get("/")]
fn login() -> Template {
    unimplemented!();
}

#[derive(FromForm)]
struct Login {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
fn login_post(login: Form<Login>, conn: DbConn) -> String {
    use schema::users::dsl::*;
    use user::UserQuery;

    let login = login.get();
    let user = match users.filter(username.eq(login.username.clone()))
        .first::<UserQuery>(&*conn) {
        Ok(user) => user,
        Err(_) => return "Oof".into(),
    };

    if let Ok(result) = bcrypt::verify(&login.password, &user.salt_hash) {
        if result {
            return "Yay".into();
        }
    }
    "Owie".into()
}
