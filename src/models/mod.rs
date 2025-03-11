use diesel::{prelude::{Insertable, Queryable}, result::Error, ExpressionMethods, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::controllers::conectar;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::filmes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Filmes{
    pub id: i32,
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
} 

#[derive(Serialize, Deserialize)]
pub struct FilmeInput{
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
} 

pub struct FilmeUpdateInput{
    pub id: i32,
    pub titulo: String,
    pub diretor: String,
    pub ano: i32,
    pub genero: String
}

pub async fn cadastrar_filme(filme: Filmes)
    -> Result<usize, String>{
    use crate::schema::filmes::dsl::*;

    let conexao: &mut diesel::MysqlConnection = &mut conectar();

    let qtd: Result<usize, Error> = diesel::insert_into(filmes)
        .values(filme)
        .execute(conexao);

    match qtd{
        Ok(qtd) => {
            return Ok(qtd)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }    
}

pub async fn buscar_filme(idfilme: i32)
    -> Result<Filmes, String>{
    use crate::schema::filmes::dsl::*;

    let conexao = &mut conectar();
    let filme = filmes
        .limit(1)
        .select(Filmes::as_select())
        .filter(id.eq(idfilme))
        .get_result( conexao);

    match filme{
        Ok(filme) => {
            return Ok(filme)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn atualizar_filme(filme: Filmes)
    -> Result<usize, String>{
    use crate::schema::filmes::dsl::*;

    let conexao = &mut conectar();

    let qtd = diesel::update(filmes)
        .set((
            titulo.eq(filme.titulo),
            genero.eq(filme.genero),
            ano.eq(filme.ano),
            diretor.eq(filme.diretor)
        ))
        .filter(id.eq(filme.id))
        .execute(conexao);

    match qtd{
        Ok(qtd) => {
            println!("{} filmes atualizados.", qtd);
            return Ok(qtd)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn buscar_todos_os_filmes()
    -> Result<Vec<Filmes>, String>{
    use crate::schema::filmes::dsl::*;

    let conexao = &mut conectar();

    let res: Result<Vec<Filmes>, Error> = filmes
        .get_results(conexao);

    match res{
        Ok(vecfilmes) => {
            return Ok(vecfilmes)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn deletar_filme(idfilme: i32)
    -> Result<usize, String>{
    use crate::schema::filmes::dsl::*;

    let conexao = &mut conectar();

    let qtd_filme_deletado = diesel::delete(
        filmes.filter(id.eq(idfilme)))
        .execute(conexao);

    match qtd_filme_deletado{
        Ok(qtd) => {
            println!("{} filmes deletados.", qtd);
            return Ok(qtd)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}