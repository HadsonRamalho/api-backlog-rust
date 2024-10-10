mod controllers;
mod routes;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = crate::routes::cria_rotas();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}