use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: Option<String>,  // Changed to Option<String>
    pub email: Option<String>, // Changed to Option<String>
    pub sessiontoken: Option<i32>, // Changed to Option<i32>
    pub password: String,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser { 
    pub name: Option<String>,  // Changed to Option<String>
    pub email: Option<String>, // Changed to Option<String>
    pub sessiontoken: Option<i32>, // Changed to Option<i32>
    pub password: String,
}
