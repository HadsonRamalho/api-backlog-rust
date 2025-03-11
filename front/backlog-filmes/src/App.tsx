import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import {Card, CardHeader, CardContent} from "../src/components/ui/card";
import {FilmeInput, atualizarFilme, buscarFilmeId, buscarFilmes, cadastrarFilme, deletarFilme} from "./interface.tsx";
import {Filme} from './interface.tsx';
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../src/components/ui/tabs";

function App() {
  const [titulo, setTitulo] = useState("");
  const [genero, setGenero] = useState("");
  const [ano, setAno] = useState(0);
  const [diretor, setDiretor] = useState("");
  const [loading, setLoading] = useState(true);
  const [vecFilmes, setVecFilmes] = useState<Filme[]>([]);

  const [idBusca, setIdBusca] = useState(0);
  const [tituloUpdate, setTituloUpdate] = useState("");
  const [generoUpdate, setGeneroUpdate] = useState("");
  const [anoUpdate, setAnoUpdate] = useState(0);
  const [diretorUpdate, setDiretorUpdate] = useState("");

  const [idDelete, setIdDelete] = useState(0);
  const [tituloDelete, setTituloDelete] = useState("");
  const [generoDelete, setGeneroDelete] = useState("");
  const [anoDelete, setAnoDelete] = useState(0);
  const [diretorDelete, setDiretorDelete] = useState("");
  const [loadingDelete, setLoadingDelete] = useState(true);
  
  const cadastraFilme = async () => {
    const filme: FilmeInput = {
      titulo: titulo,
      genero: genero,
      ano: ano,
      diretor: diretor,
    }
    const cadastrou = await cadastrarFilme(filme);
    if (cadastrou){
      alert("Filme cadastrado!");
      carregaFilmes();
    }
  }

  const carregaFilmes = async () => {
    const filmes = await buscarFilmes();
    console.log("Filmes: ", filmes);
    setVecFilmes(filmes);
  }

  const buscaFilme = async (id: number) => {
    const filme = await buscarFilmeId(id);
    console.log("Filme encontrado: ", filme);
    if(filme){
      setAnoUpdate(filme.ano);
      setDiretorUpdate(filme.diretor);
      setGeneroUpdate(filme.genero);
      setTituloUpdate(filme.titulo);

      setAnoDelete(filme.ano);
      setDiretorDelete(filme.diretor);
      setGeneroDelete(filme.genero);
      setTituloDelete(filme.titulo);
    }
  }

  const atualizaFilme = async () => {
    const filme: Filme = {
      titulo: tituloUpdate,
      genero: generoUpdate,
      ano: anoUpdate,
      diretor: diretorUpdate,
      id: idBusca
    }
    const atualizou = await atualizarFilme(filme);
    if (atualizou){
      alert("Filme atualizado!");
      setAnoUpdate(0);
      setDiretorUpdate("");
      setGeneroUpdate("");
      setTituloUpdate("");
      setIdBusca(0);
      carregaFilmes();
    }
  }

  const deletaFilme = async () => {
    const deletou = await deletarFilme(idDelete);
    if (deletou){
      alert("Filme deletado!");
      carregaFilmes();
    }
  }

  useEffect(() => {
    if(loading){
      carregaFilmes();
      setLoading(false);
    }
  }, [loading]);

  return (
    <>
      <div>
        <h1>Backlog de Filmes</h1>

        <Tabs defaultValue="cadastrar" className="w-[400px] mt-20">
          Selecione uma opção:
          <TabsList className='mt-2'>
            <TabsTrigger  value="cadastrar">Cadastrar filme</TabsTrigger>
            <TabsTrigger value="atualizar">Atualizar filme</TabsTrigger>
            <TabsTrigger value="deletar">Deletar filme</TabsTrigger>
          </TabsList>
          <TabsContent value="cadastrar">
            <Card className='bg-emerald-200'>
              <CardHeader>
                <h3>Adicionar Filme no Backlog</h3>
              </CardHeader>
              <CardContent>
              <form onSubmit={(e) => {
                e.preventDefault();
                cadastraFilme();
                }}>
                <input 
                className='rounded border-red-200 border-[20px]' 
                type='text' 
                placeholder='Título'
                required
                onChange={(e) => {setTitulo(e.target.value)}}/>
                <input type='text' 
                required
                placeholder='Diretor'
                onChange={(e) => {setDiretor(e.target.value)}}/>
                <input 
                required
                type='text' placeholder='Gênero'
                onChange={(e) => setGenero(e.target.value)}/>
                <input 
                required
                type='text' placeholder='Ano'
                onChange={(e) => {setAno(parseInt(e.target.value))}}/>
                <button className='mt-4' type='submit'>Cadastrar o filme</button>
              </form>
              </CardContent>
            </Card>
          </TabsContent>
          <TabsContent value="atualizar">
          <Card className='bg-emerald-200'>
              <CardHeader>
                <h3>Atualizar Filme</h3>
              </CardHeader>
              <CardContent>
              <form>
              <input 
                 type='text'
                 onChange={(e) => {setIdBusca(parseInt(e.target.value))}}
                 onBlur={() => {buscaFilme(idBusca)}}
                 value={idBusca}
                 placeholder='Insira o ID do filme'
                 />

                <input className='rounded border-red-200 border-[20px]'
                value={tituloUpdate}
                onChange={(e) => {setTituloUpdate(e.target.value)}}
                type='text' placeholder='Título'/>
                <input type='text' 
                value={diretorUpdate}
                placeholder='Diretor'/>
                <input type='text' 
                value={generoUpdate}
                placeholder='Gênero'/>
                <input type='text' 
                value={anoUpdate}
                placeholder='Ano'/>
                <button className='mt-4' onClick={atualizaFilme} type='button'>Atualizar o filme</button>
              </form>
              </CardContent>
            </Card>
          </TabsContent>
          <TabsContent value="deletar">
          <Card className='bg-emerald-200'>
              <CardHeader>
                <h3>Deletar Filme</h3>
              </CardHeader>
              <CardContent>
              <form>
              <input 
                 type='text'
                 onChange={(e) => {setIdDelete(parseInt(e.target.value))}}
                 onBlur={() => {buscaFilme(idDelete)}}
                 value={idDelete}
                 placeholder='Insira o ID do filme'
                 />

                <input className='rounded border-red-200 border-[20px]'
                value={tituloDelete}
                onChange={(e) => {setTituloDelete(e.target.value)}}
                type='text' placeholder='Título'/>
                <input type='text' 
                value={diretorDelete}
                placeholder='Diretor'/>
                <input type='text' 
                value={generoDelete}
                placeholder='Gênero'/>
                <input type='text' 
                value={anoDelete}
                placeholder='Ano'/>
              </form>
              </CardContent>
              <button className='mb-6' onClick={deletaFilme} type='button'>Deletar o filme</button>
            </Card>
          </TabsContent>
        </Tabs>

        
        <div>
          <Card className='bg-emerald-200 mt-10'>
            <CardHeader>Lista de Filmes</CardHeader>
            <CardContent>
              {loading ? (<>Carregando...</>)
              : (<>
              {vecFilmes.map((filme: Filme) => (
                  <Card className="bg-emerald-400 m-4" key={filme.id}>
                    <CardHeader>
                    <p>Título: {filme.titulo}</p>
                    </CardHeader>
                    <CardContent>
                    <p>Diretor: {filme.diretor}</p>
                    <p>Gênero: {filme.genero}</p>
                    <p>Ano: {filme.ano}</p>
                    <p>ID: {filme.id}</p>
                    </CardContent>
                  </Card>                
              ))}
              </>)}
            </CardContent>
          </Card>
        </div>
      </div>
    </>
  )
}

export default App
