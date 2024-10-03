use axum::{
    routing::{get, post},
    Router,
};

use crate::{create_user, root};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));
    app
}