use super::schema::people;

#[derive(FromForm)]
pub struct Registration {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
}

impl Registration {
    pub fn upgrade(self, party: i32) -> NewPerson {
        NewPerson {
            party,
            first_name:   self.first_name,
            last_name:    self.last_name,
            email:        self.email,
            phone_number: self.phone_number,
        }
    }
}

#[derive(Insertable)]
#[table_name="people"]
pub struct NewPerson {
    pub party: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
}

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub party: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
}
