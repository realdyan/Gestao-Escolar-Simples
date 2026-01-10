use rusqlite:: { Connection, Result };

pub fn inicializar_db() -> Result<Connection> {
    let conn = Connection::open("escola.db")?;

// Ativa suporte a foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

// Criação das tabelas (se não existirem)
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