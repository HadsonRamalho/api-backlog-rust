use std::env;

use axum::{extract::Path, http::StatusCode, Json};
use diesel::{query_dsl::methods::{FilterDsl, LimitDsl}, Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use diesel::query_dsl::methods::SelectDsl;

use crate::{models::{self, Filmes, NovoFilme}, schema::filmes};

pub fn conectar() -> PgConnection{
    dotenv().ok();

    let db = env::var("DATABASE_URL").expect("DATABASE_URL deveria estar definido");
    PgConnection::establish(&db).unwrap_or_else(|_| panic!("Erro ao conectar com o banco {}", db))
}

pub async fn buscar_todos_os_filmes() -> (StatusCode, axum::Json<Vec<Filmes>>) {
    use crate::schema::filmes::dsl::*;
    let conexao = &mut conectar();
    let filme = filmes
        .limit(5)
        .select(Filmes::as_select())
        .load( conexao)
        .expect("Erro carregando os filmes");
  
    (StatusCode::FOUND, axum::Json(filme))
}

pub async fn cadastrar_filme(Json(payload): Json<models::Filmes>) -> (StatusCode, axum::Json<Filmes>){
    let filme = Filmes{
        id: 1,
        titulo: payload.titulo,
        diretor: payload.diretor,
        ano: payload.ano,
        genero: payload.genero
    };
    println!("{:?}", filme);

    let conexao = &mut conectar();

    let novo_filme = NovoFilme{ id: filme.id,
        titulo: filme.titulo,
        diretor: filme.diretor,
        ano: filme.ano,
        genero: filme.genero
     };

    let filme_cadastrado = diesel::insert_into(filmes::table)
        .values(&novo_filme)
        .returning(Filmes::as_returning())
        .get_result(conexao)
        .expect("Erro ao cadastrar o filme");

    (StatusCode::CREATED, Json(filme_cadastrado))

}

pub async fn buscar_filme(Path(idfilme): Path<i32>) -> (StatusCode, axum::Json<Vec<Filmes>>){
    use crate::schema::filmes::dsl::*;
    let conexao = &mut conectar();
    let filme = filmes
        .limit(1)
        .select(Filmes::as_select())
        .filter(id.eq(idfilme))
        .load( conexao)
        .expect("Erro carregando o filme");
  
    (StatusCode::FOUND, axum::Json(filme))
}