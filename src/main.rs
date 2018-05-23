#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_contrib;
use rocket_contrib::Template;

#[macro_use] extern crate diesel;
use diesel::prelude::*;

extern crate uuid;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod database;
use database::{DbConn, init_pool};

mod schema;
mod party;
mod user;
use user::User;

#[get("/")]
fn index_user(conn: DbConn, user: User) -> Template {
    use schema::parties::dsl::*;
    use party::Party;

    let results = parties.load::<Party>(&*conn)
        .expect("Error");

    Template::render("index", json!({"parties": &results, "user": &user}))
}

use rocket::http::Cookies;
#[get("/", rank = 2)]
fn index(conn: DbConn, mut cookies: Cookies) -> Template {
    use schema::parties::dsl::*;
    use party::Party;

    let results = parties.load::<Party>(&*conn)
        .expect("Error");

    use rocket::http::Cookie;
    cookies.add_private(Cookie::new("uuid", "23a6fe73-9745-4e9a-8a73-ddb800949021"));

    Template::render("index", json!({"parties": &results}))
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, index_user])
        .mount("/party", routes![party::list, party::details, party::new, party::new_post])
        .mount("/user", routes![user::new])
        .attach(Template::fairing())
        .launch();
}
