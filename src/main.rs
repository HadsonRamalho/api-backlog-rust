use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
   
async fn home() -> String{
    "API inicializada!".to_string()
}

fn main() {
    println!("Hello, world!");
    let app: Router = Router::new().route("/", get(home));
}