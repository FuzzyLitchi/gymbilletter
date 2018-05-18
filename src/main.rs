#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::request::Form;

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
mod auth;
use auth::User;

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

#[get("/party")]
fn party() -> &'static str {
    "Parties"
}

#[get("/party/<id>")]
fn party_details(id: i32, conn: DbConn) -> Option<String> {
    use schema::parties::dsl::parties;
    use party::Party;

    match parties.find(id)
        .first::<Party>(&*conn) {
        Ok(party) => Some(format!("Title: {}\nDescription: {}", party.title, party.body)),
        Err(_) => None
    }
}

use auth::Admin;
#[get("/new_party")]
fn new_party(user: Admin) -> Template {
    Template::render("new_party", json!({"user": &user}))
}

use party::NewParty;
#[post("/new_party", data = "<new_party>")]
fn new_party_post(user: Admin, new_party: Form<NewParty>, conn: DbConn) -> String {
    use schema::parties::dsl::parties;
    use party::Party;

    match diesel::insert_into(parties)
        .values(new_party.get())
        .get_result::<Party>(&*conn) {
        Ok(party) => format!("Party \"{}\" has been created", party.title),
        Err(_) => "Error".into(),
    }
}

use auth::NewUser;
#[post("/new_user", data = "<new_user>")]
fn new_user(new_user: Form<NewUser>, conn: DbConn) -> String {
    use schema::users::dsl::users;
    use auth::{User, UserQuery};

    match diesel::insert_into(users)
        .values(new_user.get())
        .get_result::<UserQuery>(&*conn) {
        Ok(user) => format!("User \"{}\" has been created, width uuid \"{}\"", user.username, user.user_id),
        Err(_) => "Error".into(),
    }
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index_user])
        .mount("/", routes![index])
        .mount("/", routes![party])
        .mount("/", routes![party_details])
        .mount("/", routes![new_party])
        .mount("/", routes![new_party_post])
        .mount("/", routes![new_user])
        .attach(Template::fairing())
        .launch();
}
