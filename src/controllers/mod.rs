use std::env;

use axum::{extract::Path, http::StatusCode, Json};
use diesel::{query_dsl::methods::{FilterDsl, FindDsl, LimitDsl}, Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use diesel::query_dsl::methods::SelectDsl;

use crate::{models::{self, Filmes}, schema::filmes};

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
    let id: u32 = rand::random();
    let id:i32 = match id.to_string().trim().split_at(6).0.parse()
    {
        Ok(id) => id,
        Err(e) => {println!("{:?}", e);
            rand::random()
        }
    };

    let filme = Filmes{
        id,
        titulo: payload.titulo,
        diretor: payload.diretor,
        ano: payload.ano,
        genero: payload.genero
    };
    println!("{:?}", filme);

    let conexao = &mut conectar();

    let filme_cadastrado = diesel::insert_into(filmes::table)
        .values(&filme)
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

pub async fn atualizar_filme(Path(idfilme): Path<i32>, Json(payload): Json<String>) -> (StatusCode, axum::Json<Filmes>){
    use crate::schema::filmes::dsl::*;
    let conexao = &mut conectar();
    let novotitulo = payload.to_string();
    let filme = diesel::update(filmes.find(idfilme))
        .set(titulo.eq(novotitulo))
        .returning(Filmes::as_returning())
        .get_result(conexao)
        .expect("Erro ao atualizar o filme");
    (StatusCode::OK, axum::Json(filme))
}

pub async fn deletar_filme(Path(idfilme): Path<i32>) -> (StatusCode, axum::Json<String>){
    use crate::schema::filmes::dsl::*;
    let conexao = &mut conectar();
    let qtd_filme_deletado = diesel::delete(
        filmes.filter(id.eq(idfilme)))
        .execute(conexao)
        .expect("Erro ao deletar o filme");
    println!("Filmes deletados: {}", qtd_filme_deletado);
    (StatusCode::OK, axum::Json(format!("Filmes deletados: {}", qtd_filme_deletado)))
}