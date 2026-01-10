use rusqlite::{params, Connection, Result};

// Importa a struct Escola do módulo cadastros
use crate::mods::cadastros::{Escola, Professor};


pub fn inicializar_db() -> Result<Connection> {
    let conn = Connection::open("escola.db")?;

// Ativa suporte a foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

// =========== Criação das tabelas (se não existirem) ===========
    // Tabela Escola
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Escola (
                  id_escola INTEGER PRIMARY KEY,
                  nome_escola TEXT NOT NULL,
                  email TEXT UNIQUE NOT NULL,
                  telefone TEXT NOT NULL,
                  endereco TEXT NOT NULL,
                  website TEXT NOT NULL
                  )",
        [],
    )?;

    // Tabela Professor
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Professor (
                  id_professor INTEGER PRIMARY KEY,
                  nome_professor TEXT NOT NULL,
                  nomecompleto_professor TEXT NOT NULL,
                  email TEXT UNIQUE NOT NULL,
                  telefone TEXT NOT NULL,
                  endereco TEXT NOT NULL,
                  data_nascimento TEXT NOT NULL,
                  cpf TEXT UNIQUE NOT NULL,
                  data_contratacao TEXT NOT NULL,
                  data_demissao TEXT,
                  salario_inicial REAL NOT NULL,
                  salario_atual REAL NOT NULL
                  )",
        [],
    )?;

    // Tabela Aluno
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Aluno (
                  id_aluno INTEGER PRIMARY KEY,
                  nome_aluno TEXT NOT NULL,
                  nomecompleto_aluno TEXT NOT NULL,
                  email TEXT UNIQUE NOT NULL,
                  telefone TEXT NOT NULL,
                  endereco TEXT NOT NULL,
                  data_nascimento TEXT NOT NULL,
                  cpf TEXT UNIQUE NOT NULL,
                  data_matricula TEXT NOT NULL
                  )",
        [],
    )?;

    // Tabela Curso
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Curso (
                  id_curso INTEGER PRIMARY KEY,
                  nome_curso TEXT NOT NULL,
                  descricao TEXT NOT NULL,
                  duracao_horas INTEGER NOT NULL
                  )",
        [],
    )?;

    // Tabela Nível
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Nivel (
                  id_nivel INTEGER PRIMARY KEY,
                  nome_nivel TEXT NOT NULL,
                  descricao TEXT NOT NULL
                  )",
        [],
    )?;

    // Tabela Disciplina
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Disciplina (
                  id_disciplina INTEGER PRIMARY KEY,
                  nome_disciplina TEXT NOT NULL,
                  descricao TEXT NOT NULL,
                  carga_horaria INTEGER NOT NULL,
                  id_nivel INTEGER NOT NULL, FOREIGN KEY(id_nivel) REFERENCES Nivel(id_nivel)
                  )",
        [],
    )?;

    // Tabela Turma
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Turma (
                  id_turma INTEGER PRIMARY KEY,
                  nome_turma TEXT NOT NULL,
                  id_curso INTEGER NOT NULL,
                  ano INTEGER NOT NULL,
                  semestre INTEGER NOT NULL,
                  FOREIGN KEY(id_curso) REFERENCES Curso(id_curso)
                  )",
        [],
    )?;


    // Tabela Matricula
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Matricula (
                  id_matricula INTEGER PRIMARY KEY,
                  id_aluno INTEGER NOT NULL,
                  id_turma INTEGER NOT NULL,
                  data_matricula TEXT NOT NULL,
                  data_confirmacao TEXT,
                  data_cancelamento TEXT,
                  status TEXT NOT NULL,
                  FOREIGN KEY(id_aluno) REFERENCES Aluno(id_aluno),
                  FOREIGN KEY(id_turma) REFERENCES Turma(id_turma)
                  )",
        [],
    )?;

    // Tabela Alocacao_Professor
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Alocacao_Professor (
                  id_alocacao INTEGER PRIMARY KEY,
                  id_professor INTEGER NOT NULL,
                  id_turma INTEGER NOT NULL,
                  data_alocacao TEXT NOT NULL,
                  data_desalocacao TEXT,
                  FOREIGN KEY(id_professor) REFERENCES Professor(id_professor),
                  FOREIGN KEY(id_turma) REFERENCES Turma(id_turma)
                  )",
        [],
    )?;

    // Retorna a conexão com o banco de dados
    Ok(conn)
}          

// =========== CADASTRO / ALTERAÇÃO / REMOÇÃO / LISTAGEM ===========

// ESCOLA (CADASTRO / ALTERAÇÃO / REMOÇÃO / LISTAGEM)

// Função para inserir uma nova escola no db
pub fn inserir_escola(conn: &Connection, escola: &Escola) -> Result<()> {
    conn.execute(
        "INSERT INTO Escola (nome_escola, email, telefone, endereco, website) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            escola.nome_escola,
            escola.email,
            escola.telefone,
            escola.endereco,
            escola.website
        ],
    )?;
    Ok(())
}


// --- Função para atualizar os dados de uma escola existente ---
pub fn atualizar_escola_db(conn: &Connection, id: i32, escola: &Escola) -> Result<()> {
    let mut stmt = conn.prepare(
        "UPDATE Escola SET nome_escola = ?1, email = ?2, telefone = ?3, endereco = ?4, website = ?5 
         WHERE id_escola = ?6"
    )?;

    stmt.execute(params![
        escola.nome_escola,
        escola.email,
        escola.telefone,
        escola.endereco,
        escola.website,
        id
    ])?;

    Ok(())
}

// -- Função para apagaruma escola existente --
pub fn remover_escola_db(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Escola WHERE id_escola = ?1", params![id])?;
    Ok(())
}

// -- Função para listar todas as escolas cadastradas --
pub fn listar_escolas_db(conn: &Connection) -> Result<Vec<Escola>> {
    let mut stmt = conn.prepare("SELECT id_escola, nome_escola, email, telefone, endereco, website FROM Escola")?;
    let escola_iter = stmt.query_map([], |row| {
        Ok(Escola {
            id_escola: row.get(0)?,
            nome_escola: row.get(1)?,
            email: row.get(2)?,
            telefone: row.get(3)?,
            endereco: row.get(4)?,
            website: row.get(5)?,
        })
    })?;

    let mut escolas = Vec::new();
    for escola in escola_iter {
        escolas.push(escola?);
    }
    Ok(escolas)
}

// =====================================

// PROFESSOR (CADASTRO / ALTERAÇÃO / REMOÇÃO / LISTAGEM)

// -- Função para inserir um novo professor no db --
pub fn inserir_professor(conn: &Connection, p: &Professor) -> Result<()> {
    conn.execute(
        "INSERT INTO Professor (nome_professor, nomecompleto_professor, email, telefone, endereco, 
         data_nascimento, cpf, data_contratacao, salario_inicial, salario_atual) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)", // Usando salário inicial = atual na inserção
        params![
            p.nome_professor, p.nome_completo, p.email, p.telefone, 
            p.endereco, p.data_nascimento, p.cpf, p.data_contratacao, p.salario_atual
        ],
    )?;
    Ok(())
}

// -- Função para listar Professores -
pub fn listar_professores_db(conn: &Connection) -> Result<Vec<Professor>> {
    let mut stmt = conn.prepare("SELECT id_professor, nome_professor, email, cpf, salario_atual FROM Professor")?;
    let prof_iter = stmt.query_map([], |row| {
        Ok(Professor {
            id_professor: Some(row.get(0)?),
            nome_professor: row.get(1)?,
            nome_completo: String::new(), // Simplificado para a lista
            email: row.get(2)?,
            telefone: String::new(),
            endereco: String::new(),
            data_nascimento: String::new(),
            cpf: row.get(3)?,
            data_contratacao: String::new(),
            salario_atual: row.get(4)?,
        })
    })?;

    let mut professores = Vec::new();
    for p in prof_iter { professores.push(p?); }
    Ok(professores)
}

pub fn atualizar_professor_db(conn: &Connection, id: i32, p: &Professor) -> Result<()> {
    conn.execute(
        "UPDATE Professor SET nome_professor = ?1, nomecompleto_professor = ?2, email = ?3, 
         telefone = ?4, endereco = ?5, data_nascimento = ?6, cpf = ?7, data_contratacao = ?8, 
         salario_atual = ?9 WHERE id_professor = ?10",
        params![
            p.nome_professor, p.nome_completo, p.email, p.telefone, 
            p.endereco, p.data_nascimento, p.cpf, p.data_contratacao, 
            p.salario_atual, id
        ],
    )?;
    Ok(())
}

// -- Função para remover um professor --
pub fn remover_professor_db(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Professor WHERE id_professor = ?1", params![id])?;
    Ok(())
}



// =====================================

// =========== 