use axum::{
    http::Method, routing::{delete, get, post, put}, Router
};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::{atualizar_filme, buscar_filme, buscar_todos_os_filmes, cadastrar_filme, deletar_filme};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/buscar_todos_os_filmes", get(buscar_todos_os_filmes))
        .route("/cadastrar_filme", post(cadastrar_filme))
        .route("/buscar_filme/", get(buscar_filme))
        .route("/atualizar_filme", put(atualizar_filme))
        .route("/deletar_filme/", delete(deletar_filme))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE, Method::PATCH])
            .allow_headers(Any))
        ;
    app
}