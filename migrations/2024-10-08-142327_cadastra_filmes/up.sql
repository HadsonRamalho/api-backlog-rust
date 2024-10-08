-- Your SQL goes here
CREATE TABLE filmes(
    id INT PRIMARY KEY,
    titulo VARCHAR(64) NOT NULL,
    diretor VARCHAR(64) NOT NULL,
    ano INT NOT NULL,
    genero VARCHAR(64) NOT NULL
)