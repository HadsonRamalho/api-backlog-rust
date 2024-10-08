// @generated automatically by Diesel CLI.

diesel::table! {
    filmes (id) {
        id -> Int4,
        #[max_length = 64]
        titulo -> Varchar,
        #[max_length = 64]
        diretor -> Varchar,
        ano -> Int4,
        #[max_length = 64]
        genero -> Varchar,
    }
}
