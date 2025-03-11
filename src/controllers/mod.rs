use std::env;

use axum::{extract::{Path, Query}, http::StatusCode, Json};
use diesel::{query_dsl::methods::{FilterDsl, FindDsl, LimitDsl}, Connection, ExpressionMethods, MysqlConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use diesel::query_dsl::methods::SelectDsl;
use serde::{Deserialize, Serialize};

use crate::{models::{self, FilmeInput, Filmes}, schema::filmes};

pub fn conectar() -> MysqlConnection{
    dotenv().ok();

    let db = env::var("DATABASE_URL").expect("DATABASE_URL deveria estar definido");
    MysqlConnection::establish(&db).unwrap_or_else(|_| panic!("Erro ao conectar com o banco {}", db))
}

pub async fn buscar_todos_os_filmes()
    -> Result<(StatusCode, Json<Vec<Filmes>>), (StatusCode, Json<String>)>{
    match models::buscar_todos_os_filmes().await{
        Ok(filmes) => {
            return Ok((StatusCode::OK, Json(filmes)));
        },
        Err(e) =>{
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn cadastrar_filme(input: Json<FilmeInput>) 
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.ano.to_string().trim().is_empty() 
        || input.diretor.trim().is_empty() || input.genero.trim().is_empty()
        || input.titulo.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    
    let id: u32 = rand::random();
    let id:i32 = match id.to_string().trim().split_at(6).0.parse(){
        Ok(id) => id,
        Err(e) => {
            println!("Erro ao gerar rand: {:?}", e);
            rand::random()
        }
    };

    let filme = Filmes{
        id,
        titulo: input.titulo.to_string(),
        diretor: input.diretor.to_string(),
        ano: input.ano,
        genero: input.genero.to_string(),
    };

    match models::cadastrar_filme(filme).await{
        Ok(_) => {
            return Ok((StatusCode::OK, Json("Filme cadastrado.".to_string())))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IdInput{
    id: i32
}

pub async fn buscar_filme(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Filmes>), (StatusCode, Json<String>)>{
    if id.id.to_string().trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = id.id;

    match models::buscar_filme(id).await{
        Ok(filme) => {
            return Ok((StatusCode::OK, Json(filme)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn atualizar_filme(input: Json<Filmes>)
    -> Result<StatusCode, (StatusCode, Json<String>)>{
    if input.titulo.trim().is_empty() || input.genero.trim().is_empty()
        || input.ano.to_string().trim().is_empty() || input.diretor.trim().is_empty()
        || input.id.to_string().trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    match models::atualizar_filme(input.0).await{
        Ok(_) => {
            return Ok(StatusCode::OK)
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn deletar_filme(Query(id): Query<IdInput>)
    -> Result<StatusCode, (StatusCode, Json<String>)>{
    if id.id.to_string().trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = id.id;

    match models::deletar_filme(id).await{
        Ok(_) => {
            return Ok(StatusCode::OK)
        },
        Err(e) =>{
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}