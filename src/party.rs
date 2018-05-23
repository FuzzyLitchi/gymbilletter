use super::schema::parties;

use diesel;
use diesel::prelude::*;

use rocket::request::Form;
use rocket_contrib::Template;

use database::DbConn;
use user::Admin;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Party {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, FromForm)]
#[table_name="parties"]
pub struct NewParty {
    pub title: String,
    pub body: String,
}

#[get("/")]
fn list () -> &'static str {
    "Parties"
}

#[get("/<id>")]
fn details(id: i32, conn: DbConn) -> Option<String> {
    match parties::dsl::parties.find(id)
        .first::<Party>(&*conn) {
        Ok(party) => Some(format!("Title: {}\nDescription: {}", party.title, party.body)),
        Err(_) => None
    }
}

#[get("/new")]
fn new(user: Admin) -> Template {
    Template::render("new_party", json!({"user": &user}))
}

#[post("/new", data = "<new_party>")]
fn new_post(_user: Admin, new_party: Form<NewParty>, conn: DbConn) -> String {
    match diesel::insert_into(parties::dsl::parties)
        .values(new_party.get())
        .get_result::<Party>(&*conn) {
        Ok(party) => format!("Party \"{}\" has been created", party.title),
        Err(_) => "Error".into(),
    }
}
