// @generated automatically by Diesel CLI.

diesel::table! {
    filmes (id) {
        id -> Integer,
        #[max_length = 64]
        titulo -> Varchar,
        #[max_length = 64]
        diretor -> Varchar,
        ano -> Integer,
        #[max_length = 64]
        genero -> Varchar,
    }
}
