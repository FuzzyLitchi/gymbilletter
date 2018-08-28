//This file is for the admin interface
use rocket::request::Form;
use rocket::http::{Cookies, Cookie};
use rocket_contrib::Template;

use database::DbConn;
use bcrypt;

use diesel::prelude::*;

#[get("/login")]
fn login() -> Template {
    Template::render("login", json!({}))
}

#[derive(FromForm)]
struct Login {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
fn login_post(login: Form<Login>, conn: DbConn, mut cookies: Cookies) -> String {
    use schema::users::dsl::*;
    use user::UserQuery;

    let login = login.get();
    let user = match users.filter(username.eq(login.username.clone()))
        .first::<UserQuery>(&*conn) {
        Ok(user) => user,
        Err(_) => return "Oof".into(),
    };

    if let Ok(result) = bcrypt::verify(&login.password, &user.hash) {
        if result {
            cookies.add_private(Cookie::new("uuid", user.user_id.hyphenated().to_string()));
            return "Yay".into();
        }
    }
    "Owie".into()
}
