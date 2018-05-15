use super::schema::parties;
use serde::ser::*;

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
