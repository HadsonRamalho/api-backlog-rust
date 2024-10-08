use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::{atualizar_filme, buscar_filme, buscar_todos_os_filmes}, root};
use crate::controllers::cadastrar_filme;

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        // `GET /` goes to `root`
        .route("/", get(buscar_todos_os_filmes))
        // `POST /users` goes to `create_user`
        .route("/", post(cadastrar_filme))
        .route("/:id", get(buscar_filme))
        .route("/atualizar/:id", get(atualizar_filme))
        ;
    app
}