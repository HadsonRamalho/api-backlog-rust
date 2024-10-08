use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::filmes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Filmes{
    pub id: i32,
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
} 