use diesel::prelude::*;
use crate::schema::*;

#[derive(serde::Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
