use rusqlite::{params, Connection, Result};

// ==================================================
// ENTIDADES DO ERP (MODELOS DE DADOS)
// ==================================================

#[derive(Debug, Clone)]
pub struct Escola {
    pub id_escola: Option<i32>,
    pub nome_escola: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub website: String,
}

#[derive(Debug, Clone)]
pub struct Professor {
    pub id_professor: Option<i32>,
    pub id_escola: i32,
    pub nome_professor: String,
    pub nome_completo: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub data_nascimento: String,
    pub cpf: String,
    pub data_contratacao: String,
    pub salario_atual: f64,
}

#[derive(Debug, Clone)]
pub struct Aluno {
    pub id_aluno: Option<i32>,
    pub id_escola: i32,
    pub nome_aluno: String,
    pub nome_completo: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub data_nascimento: String,
    pub cpf: String,
    pub data_matricula: String,
}

#[derive(Debug, Clone)]
pub struct Curso {
    pub id_curso: Option<i32>,
    pub id_escola: i32,
    pub nome_curso: String,
    pub descricao: String,
    pub duracao_horas: i32,
}

// 🔧 CORREÇÃO: Campo nome_curso adicionado para compatibilidade com UI
#[derive(Debug, Clone)]
pub struct Turma {
    #[allow(dead_code)] // Usado em queries SQL e na função buscar_turma_por_id
    pub id_turma: Option<i32>,
    pub id_escola: i32,
    pub nome_turma: String,
    pub id_curso: i32,
    pub ano: i32,
    pub semestre: i32,
}

// Struct auxiliar para a UI que inclui o nome do curso
#[derive(Debug, Clone)]
pub struct TurmaInfo {
    pub id_turma: i32,
    pub nome_turma: String,
    pub id_curso: i32,
    pub nome_curso: String,
    pub ano: i32,
    pub semestre: i32,
}

#[derive(Debug, Clone)]
pub struct Disciplina {
    pub id_disciplina: Option<i32>,
    pub id_escola: i32,
    pub nome_disciplina: String,
    pub descricao: String,
    pub carga_horaria: i32,
}

#[derive(Debug, Clone)]
pub struct MatriculaInfo {
    pub id_matricula: i32,
    pub nome_aluno: String,
    pub nome_turma: String,
    pub data_matricula: String,
    pub status: String,
}

// ==================================================
// INICIALIZAÇÃO DO BANCO
// ==================================================

pub fn inicializar_db() -> Result<Connection> {
    let conn = Connection::open("escola.db")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    conn.execute_batch("
        BEGIN;
        CREATE TABLE IF NOT EXISTS Escola (
            id_escola INTEGER PRIMARY KEY, nome_escola TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL, telefone TEXT NOT NULL,
            endereco TEXT NOT NULL, website TEXT
        );
        CREATE TABLE IF NOT EXISTS Curso (
            id_curso INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            nome_curso TEXT NOT NULL, descricao TEXT, duracao_horas INTEGER NOT NULL,
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS Professor (
            id_professor INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            nome_professor TEXT NOT NULL, nome_completo TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL, telefone TEXT NOT NULL, endereco TEXT NOT NULL,
            data_nascimento TEXT NOT NULL, cpf TEXT UNIQUE NOT NULL,
            data_contratacao TEXT NOT NULL, salario_atual REAL NOT NULL,
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS Aluno (
            id_aluno INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            nome_aluno TEXT NOT NULL, nome_completo TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL, telefone TEXT NOT NULL, endereco TEXT NOT NULL,
            data_nascimento TEXT NOT NULL, cpf TEXT UNIQUE NOT NULL, data_matricula TEXT NOT NULL,
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS Turma (
            id_turma INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            nome_turma TEXT NOT NULL, id_curso INTEGER NOT NULL,
            ano INTEGER NOT NULL, semestre INTEGER NOT NULL,
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE,
            FOREIGN KEY(id_curso) REFERENCES Curso(id_curso) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS Disciplina (
            id_disciplina INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            nome_disciplina TEXT NOT NULL, descricao TEXT, carga_horaria INTEGER NOT NULL,
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS Matricula (
            id_matricula INTEGER PRIMARY KEY, id_escola INTEGER NOT NULL,
            id_aluno INTEGER NOT NULL, id_turma INTEGER NOT NULL,
            data_matricula TEXT NOT NULL, status TEXT DEFAULT 'ATIVA',
            FOREIGN KEY(id_escola) REFERENCES Escola(id_escola) ON DELETE CASCADE,
            FOREIGN KEY(id_aluno) REFERENCES Aluno(id_aluno) ON DELETE CASCADE,
            FOREIGN KEY(id_turma) REFERENCES Turma(id_turma) ON DELETE CASCADE
        );
        COMMIT;
    ")?;
    Ok(conn)
}

// ==================================================
// CRUD ESCOLA
// ==================================================

pub fn inserir_escola(conn: &Connection, e: &Escola) -> Result<i64> {
    conn.execute("INSERT INTO Escola (nome_escola, email, telefone, endereco, website) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![e.nome_escola, e.email, e.telefone, e.endereco, e.website])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_escolas(conn: &Connection) -> Result<Vec<Escola>> {
    let mut stmt = conn.prepare("SELECT id_escola, nome_escola, email, telefone, endereco, website FROM Escola")?;
    let rows = stmt.query_map([], |row| Ok(Escola {
        id_escola: Some(row.get(0)?), nome_escola: row.get(1)?, email: row.get(2)?,
        telefone: row.get(3)?, endereco: row.get(4)?, website: row.get(5)?,
    }))?;
    rows.collect()
}

pub fn atualizar_escola(conn: &Connection, id: i32, e: &Escola) -> Result<()> {
    conn.execute("UPDATE Escola SET nome_escola=?1, email=?2, telefone=?3, endereco=?4, website=?5 WHERE id_escola=?6",
        params![e.nome_escola, e.email, e.telefone, e.endereco, e.website, id])?;
    Ok(())
}

pub fn remover_escola(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Escola WHERE id_escola = ?1", params![id])?;
    Ok(())
}

pub fn buscar_escola_por_id(conn: &Connection, id: i32) -> Result<Escola> {
    conn.query_row("SELECT id_escola, nome_escola, email, telefone, endereco, website FROM Escola WHERE id_escola = ?1",
        params![id], |row| Ok(Escola {
            id_escola: Some(row.get(0)?), nome_escola: row.get(1)?, email: row.get(2)?,
            telefone: row.get(3)?, endereco: row.get(4)?, website: row.get(5)?,
        }))
}

// ==================================================
// CRUD PROFESSOR
// ==================================================

pub fn inserir_professor(conn: &Connection, p: &Professor) -> Result<i64> {
    conn.execute("INSERT INTO Professor (id_escola, nome_professor, nome_completo, email, telefone, endereco, data_nascimento, cpf, data_contratacao, salario_atual) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![p.id_escola, p.nome_professor, p.nome_completo, p.email, p.telefone, p.endereco, p.data_nascimento, p.cpf, p.data_contratacao, p.salario_atual])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_professores(conn: &Connection, id_escola: i32) -> Result<Vec<Professor>> {
    let mut stmt = conn.prepare("SELECT id_professor, id_escola, nome_professor, nome_completo, email, telefone, endereco, data_nascimento, cpf, data_contratacao, salario_atual FROM Professor WHERE id_escola = ?1")?;
    let rows = stmt.query_map([id_escola], |row| Ok(Professor {
        id_professor: Some(row.get(0)?), id_escola: row.get(1)?, nome_professor: row.get(2)?, nome_completo: row.get(3)?,
        email: row.get(4)?, telefone: row.get(5)?, endereco: row.get(6)?, data_nascimento: row.get(7)?,
        cpf: row.get(8)?, data_contratacao: row.get(9)?, salario_atual: row.get(10)?,
    }))?;
    rows.collect()
}

pub fn atualizar_professor(conn: &Connection, id: i32, p: &Professor) -> Result<()> {
    conn.execute("UPDATE Professor SET nome_professor=?1, nome_completo=?2, email=?3, telefone=?4, endereco=?5, data_nascimento=?6, cpf=?7, data_contratacao=?8, salario_atual=?9 WHERE id_professor=?10",
        params![p.nome_professor, p.nome_completo, p.email, p.telefone, p.endereco, p.data_nascimento, p.cpf, p.data_contratacao, p.salario_atual, id])?;
    Ok(())
}

pub fn remover_professor(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Professor WHERE id_professor = ?1", params![id])?;
    Ok(())
}

// ==================================================
// CRUD ALUNO
// ==================================================

pub fn inserir_aluno(conn: &Connection, a: &Aluno) -> Result<i64> {
    conn.execute("INSERT INTO Aluno (id_escola, nome_aluno, nome_completo, email, telefone, endereco, data_nascimento, cpf, data_matricula) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![a.id_escola, a.nome_aluno, a.nome_completo, a.email, a.telefone, a.endereco, a.data_nascimento, a.cpf, a.data_matricula])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_alunos(conn: &Connection, id_escola: i32) -> Result<Vec<Aluno>> {
    let mut stmt = conn.prepare("SELECT id_aluno, id_escola, nome_aluno, nome_completo, email, telefone, endereco, data_nascimento, cpf, data_matricula FROM Aluno WHERE id_escola = ?1")?;
    let rows = stmt.query_map([id_escola], |row| Ok(Aluno {
        id_aluno: Some(row.get(0)?), id_escola: row.get(1)?, nome_aluno: row.get(2)?, nome_completo: row.get(3)?,
        email: row.get(4)?, telefone: row.get(5)?, endereco: row.get(6)?, data_nascimento: row.get(7)?,
        cpf: row.get(8)?, data_matricula: row.get(9)?,
    }))?;
    rows.collect()
}

pub fn atualizar_aluno(conn: &Connection, id: i32, a: &Aluno) -> Result<()> {
    conn.execute("UPDATE Aluno SET nome_aluno=?1, nome_completo=?2, email=?3, telefone=?4, endereco=?5, data_nascimento=?6, cpf=?7, data_matricula=?8 WHERE id_aluno=?9",
        params![a.nome_aluno, a.nome_completo, a.email, a.telefone, a.endereco, a.data_nascimento, a.cpf, a.data_matricula, id])?;
    Ok(())
}

pub fn remover_aluno(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Aluno WHERE id_aluno = ?1", params![id])?;
    Ok(())
}

// 🔧 NOVA FUNÇÃO: Buscar aluno por ID para edição
pub fn buscar_aluno_por_id(conn: &Connection, id: i32) -> Result<Aluno> {
    conn.query_row(
        "SELECT id_aluno, id_escola, nome_aluno, nome_completo, email, telefone, endereco, data_nascimento, cpf, data_matricula 
         FROM Aluno WHERE id_aluno = ?1",
        params![id],
        |row| Ok(Aluno {
            id_aluno: Some(row.get(0)?),
            id_escola: row.get(1)?,
            nome_aluno: row.get(2)?,
            nome_completo: row.get(3)?,
            email: row.get(4)?,
            telefone: row.get(5)?,
            endereco: row.get(6)?,
            data_nascimento: row.get(7)?,
            cpf: row.get(8)?,
            data_matricula: row.get(9)?,
        })
    )
}

// ==================================================
// CRUD CURSO
// ==================================================

pub fn inserir_curso(conn: &Connection, c: &Curso) -> Result<i64> {
    conn.execute("INSERT INTO Curso (id_escola, nome_curso, descricao, duracao_horas) VALUES (?1, ?2, ?3, ?4)",
        params![c.id_escola, c.nome_curso, c.descricao, c.duracao_horas])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_cursos(conn: &Connection, id_escola: i32) -> Result<Vec<Curso>> {
    let mut stmt = conn.prepare("SELECT id_curso, id_escola, nome_curso, descricao, duracao_horas FROM Curso WHERE id_escola = ?1")?;
    let rows = stmt.query_map([id_escola], |row| Ok(Curso {
        id_curso: Some(row.get(0)?), id_escola: row.get(1)?, nome_curso: row.get(2)?,
        descricao: row.get(3)?, duracao_horas: row.get(4)?,
    }))?;
    rows.collect()
}

pub fn atualizar_curso(conn: &Connection, id: i32, c: &Curso) -> Result<()> {
    conn.execute("UPDATE Curso SET nome_curso=?1, descricao=?2, duracao_horas=?3 WHERE id_curso=?4",
        params![c.nome_curso, c.descricao, c.duracao_horas, id])?;
    Ok(())
}

pub fn remover_curso(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Curso WHERE id_curso = ?1", params![id])?;
    Ok(())
}

// ==================================================
// CRUD TURMA (FORTALECIDO COM JOIN)
// ==================================================

pub fn inserir_turma(conn: &Connection, t: &Turma) -> Result<i64> {
    conn.execute("INSERT INTO Turma (id_escola, nome_turma, id_curso, ano, semestre) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![t.id_escola, t.nome_turma, t.id_curso, t.ano, t.semestre])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_turmas(conn: &Connection, id_escola: i32) -> Result<Vec<TurmaInfo>> {
    let mut stmt = conn.prepare("
        SELECT t.id_turma, t.nome_turma, t.id_curso, c.nome_curso, t.ano, t.semestre 
        FROM Turma t
        JOIN Curso c ON t.id_curso = c.id_curso
        WHERE t.id_escola = ?1")?;
    let rows = stmt.query_map([id_escola], |row| Ok(TurmaInfo {
        id_turma: row.get(0)?,
        nome_turma: row.get(1)?,
        id_curso: row.get(2)?,
        nome_curso: row.get(3)?,
        ano: row.get(4)?,
        semestre: row.get(5)?,
    }))?;
    rows.collect()
}

pub fn atualizar_turma(conn: &Connection, id: i32, t: &Turma) -> Result<()> {
    conn.execute("UPDATE Turma SET nome_turma=?1, id_curso=?2, ano=?3, semestre=?4 WHERE id_turma=?5",
        params![t.nome_turma, t.id_curso, t.ano, t.semestre, id])?;
    Ok(())
}

pub fn remover_turma(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Turma WHERE id_turma = ?1", params![id])?;
    Ok(())
}

// 🔧 NOVA FUNÇÃO: Buscar turma por ID para edição
pub fn buscar_turma_por_id(conn: &Connection, id: i32) -> Result<Turma> {
    conn.query_row(
        "SELECT id_turma, id_escola, nome_turma, id_curso, ano, semestre
         FROM Turma
         WHERE id_turma = ?1",
        params![id],
        |row| Ok(Turma {
            id_turma: Some(row.get(0)?),
            id_escola: row.get(1)?,
            nome_turma: row.get(2)?,
            id_curso: row.get(3)?,
            ano: row.get(4)?,
            semestre: row.get(5)?,
        })
    )
}

// ==================================================
// CRUD DISCIPLINA
// ==================================================

pub fn inserir_disciplina(conn: &Connection, d: &Disciplina) -> Result<i64> {
    conn.execute("INSERT INTO Disciplina (id_escola, nome_disciplina, descricao, carga_horaria) VALUES (?1, ?2, ?3, ?4)",
        params![d.id_escola, d.nome_disciplina, d.descricao, d.carga_horaria])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_disciplinas(conn: &Connection, id_escola: i32) -> Result<Vec<Disciplina>> {
    let mut stmt = conn.prepare("SELECT id_disciplina, id_escola, nome_disciplina, descricao, carga_horaria FROM Disciplina WHERE id_escola = ?1")?;
    let rows = stmt.query_map([id_escola], |row| Ok(Disciplina {
        id_disciplina: Some(row.get(0)?), id_escola: row.get(1)?, nome_disciplina: row.get(2)?,
        descricao: row.get(3)?, carga_horaria: row.get(4)?,
    }))?;
    rows.collect()
}

pub fn atualizar_disciplina(conn: &Connection, id: i32, d: &Disciplina) -> Result<()> {
    conn.execute("UPDATE Disciplina SET nome_disciplina=?1, descricao=?2, carga_horaria=?3 WHERE id_disciplina=?4",
        params![d.nome_disciplina, d.descricao, d.carga_horaria, id])?;
    Ok(())
}

pub fn remover_disciplina(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Disciplina WHERE id_disciplina = ?1", params![id])?;
    Ok(())
}

// ==================================================
// MATRÍCULAS
// ==================================================

pub fn inserir_matricula(conn: &Connection, id_escola: i32, id_aluno: i32, id_turma: i32, data: &str) -> Result<i64> {
    conn.execute("INSERT INTO Matricula (id_escola, id_aluno, id_turma, data_matricula, status) VALUES (?1, ?2, ?3, ?4, 'ATIVA')",
        params![id_escola, id_aluno, id_turma, data])?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_matriculas(conn: &Connection, id_escola: i32) -> Result<Vec<MatriculaInfo>> {
    let mut stmt = conn.prepare(
        "SELECT m.id_matricula, a.nome_aluno, t.nome_turma, m.data_matricula, m.status 
         FROM Matricula m
         JOIN Aluno a ON m.id_aluno = a.id_aluno
         JOIN Turma t ON m.id_turma = t.id_turma
         WHERE m.id_escola = ?1"
    )?;
    let rows = stmt.query_map([id_escola], |row| Ok(MatriculaInfo {
        id_matricula: row.get(0)?,
        nome_aluno: row.get(1)?,
        nome_turma: row.get(2)?,
        data_matricula: row.get(3)?,
        status: row.get(4)?,
    }))?;
    rows.collect()
}

pub fn remover_matricula(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM Matricula WHERE id_matricula = ?1", params![id])?;
    Ok(())
}