use axum::{
    routing::{get, post, put},
    Router,
};

use crate::controllers::{atualizar_filme, buscar_filme, buscar_todos_os_filmes, deletar_filme};
use crate::controllers::cadastrar_filme;

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/", get(buscar_todos_os_filmes))
        .route("/", post(cadastrar_filme))
        .route("/:id", get(buscar_filme))
        .route("/atualizar/:id", put(atualizar_filme))
        .route("/deletar/:id", get(deletar_filme));
    app
}