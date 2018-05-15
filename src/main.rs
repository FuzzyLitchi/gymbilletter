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

#[get("/")]
fn index(conn: DbConn) -> Template {
    use schema::parties::dsl::*;
    use party::Party;

    let results = parties.load::<Party>(&*conn)
        .expect("Error");

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

use party::NewParty;
#[post("/new_party", data = "<new_party>")]
fn new_party(new_party: Form<NewParty>, conn: DbConn) -> String {
    use schema::parties::dsl::parties;
    use party::Party;

    match diesel::insert_into(parties)
        .values(new_party.get())
        .get_result::<Party>(&*conn) {
        Ok(party) => format!("Party \"{}\" has been created", party.title),
        Err(_) => "Error".into(),
    }
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index])
        .mount("/", routes![party])
        .mount("/", routes![party_details])
        .mount("/", routes![new_party])
        .attach(Template::fairing())
        .launch();
}
