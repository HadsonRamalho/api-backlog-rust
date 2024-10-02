use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use mysql_async::{params, prelude::Queryable, Pool};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let resultado = cria_tabelas().await;
    println!("{:?}", resultado);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn cria_tabelas() -> Result<(), mysql_async::Error> {
    let pool = create_pool().await?;
    let mut conn = pool.get_conn().await?;
    let result = conn.exec_drop("CREATE TABLE IF NOT EXISTS filmes ( id INT AUTO_INCREMENT PRIMARY KEY, titulo VARCHAR(50) NOT NULL)", {}).await;
    match result{
        Ok(_) => {
            println!("BANCO CRIADO");
            return Ok(())
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(e)))
        }
    }
}

pub async fn create_pool() -> Result<Pool, mysql_async::Error> {
    let url = format!("mysql://root:0110@127.0.0.1:3307/apibacklogrust"); // A porta pode ser 3306 em outras máquinas; A senha pode ser diferente
    println!("{}", url);
    let pool = Pool::from_url(url);
    pool
}