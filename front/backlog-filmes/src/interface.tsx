export interface Filme{
    id: number;
    titulo: string;
    diretor: string;
    genero: string;
    ano: number;
}

export interface FilmeInput{
    titulo: string;
    diretor: string;
    genero: string;
    ano: number;
}

export async function buscarFilmes() : Promise<Filme[]>{
    try{
        const response = await fetch('http://localhost:3000/buscar_todos_os_filmes');
        if(!response.ok){
            const msg = await response.json();
            throw new Error(msg);
        }
        const vec = await response.json();
        return vec;
    } catch(error){
        console.error('Erro ao carregar: ', error);
    }
    return [];
}

export async function cadastrarFilme(filme: FilmeInput){
    try{
        const response = await fetch('http://localhost:3000/cadastrar_filme',
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                titulo: filme.titulo,
                genero: filme.genero,
                ano: filme.ano,
                diretor: filme.diretor
            })
        });
        if(!response.ok){
            const msg = await response.json();
            throw new Error(msg);
        }
        return true;
    } catch(error){
        console.error('Erro ao cadastrar o filme: ', error);
        return false;
    }
}

export async function buscarFilmeId(id: number) : Promise<Filme | null>{
    try{
        const response = await fetch(`http://localhost:3000/buscar_filme/?id=${encodeURIComponent(id)}`,{
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            },
        });
        if(!response.ok){
            const msg = await response.json();
            throw new Error(msg);
        }
        const filme = await response.json();
        return filme;
    } catch(error){
        console.error('Erro ao buscar: ', error);
    }
    return null;
}

export async function atualizarFilme(filme: Filme){
    try{
        const response = await fetch('http://localhost:3000/atualizar_filme',
        {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                titulo: filme.titulo,
                genero: filme.genero,
                ano: filme.ano,
                diretor: filme.diretor,
                id: filme.id
            })
        });
        if(!response.ok){
            const msg = await response.json();
            throw new Error(msg);
        }
        return true;
    } catch(error){
        console.error('Erro ao atualizar o filme: ', error);
        return false;
    }
}

export async function deletarFilme(id: number) : Promise<boolean>{
    try{
        const response = await fetch(`http://localhost:3000/deletar_filme/?id=${encodeURIComponent(id)}`,{
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json'
            },
        });
        if(!response.ok){
            const msg = await response.json();
            throw new Error(msg);
        }
        return true;
    } catch(error){
        console.error('Erro ao buscar: ', error);
    }
    return false;
}