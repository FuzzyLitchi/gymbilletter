#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;

extern crate rocket_contrib;
use rocket_contrib::Template;

#[macro_use] extern crate diesel;
use diesel::prelude::*;

extern crate uuid;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate bcrypt;
pub const DEFAULT_COST: u32 = 8;

mod database;
use database::{DbConn, init_pool};

mod schema;
mod party;
mod user;
use user::User;
mod people;
mod admin;

use std::path::{Path, PathBuf};

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
    //This is a debug thing
    cookies.add_private(Cookie::new("uuid", "d5e9e5be-861c-4455-b7df-f42f1b01da80"));

    Template::render("index", json!({"parties": &results}))
}

#[get("/<file..>", rank=10)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, index_user, files])
        .mount("/party", routes![party::list, party::details, party::sign_up, party::new, party::new_post])
        .mount("/user", routes![user::new])
        .mount("/admin", routes![admin::login, admin::login_post])
        .attach(Template::fairing())
        .launch();
}
