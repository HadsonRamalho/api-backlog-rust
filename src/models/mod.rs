use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::filmes;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::filmes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Filmes{
    pub id: i32,
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
}   

#[derive(Insertable)]
#[diesel(table_name = crate::schema::filmes)]
pub struct NovoFilme{
    pub id: i32,
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
}