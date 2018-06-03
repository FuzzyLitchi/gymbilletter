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
fn details(id: i32, conn: DbConn) -> Option<Template> {
    match parties::dsl::parties.find(id)
        .first::<Party>(&*conn) {
        Ok(party) => Some(Template::render("party", json!({"party": &party}))),
        Err(_) => None
    }
}

use people::Registration;
#[post("/<id>", data = "<registration>")]
fn sign_up(id: i32, registration: Form<Registration>, conn: DbConn) -> String {
    use super::schema::people::dsl::people;
    use people::Person;

    match diesel::insert_into(people)
        .values(registration.into_inner().upgrade(id))
        .get_result::<Person>(&*conn) {
            Ok(person) => format!("{} has been added to the party.", person.first_name),
            Err(_) => "Error".into(),
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
