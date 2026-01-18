use rusqlite::{params, Connection, Result};
use crate::db::{Aluno, Professor, TurmaInfo, Curso, Disciplina, MatriculaInfo};

// ==================================================
// PAGINAÇÃO + FILTROS - ALUNOS
// ==================================================

pub fn listar_alunos_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<Aluno>, i32)> {
    // Conta total de registros
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Aluno WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Aluno WHERE id_escola = ?1 
         AND (nome_aluno LIKE ?2 OR cpf LIKE ?2 OR email LIKE ?2)"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    // Busca registros paginados
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT id_aluno, id_escola, nome_aluno, nome_completo, email, telefone, 
                endereco, data_nascimento, cpf, data_matricula 
         FROM Aluno 
         WHERE id_escola = ?1 
         ORDER BY nome_aluno 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT id_aluno, id_escola, nome_aluno, nome_completo, email, telefone, 
                endereco, data_nascimento, cpf, data_matricula 
         FROM Aluno 
         WHERE id_escola = ?1 
         AND (nome_aluno LIKE ?4 OR cpf LIKE ?4 OR email LIKE ?4)
         ORDER BY nome_aluno 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let alunos = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(Aluno {
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
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(Aluno {
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
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((alunos, total))
}

// ==================================================
// PAGINAÇÃO + FILTROS - PROFESSORES
// ==================================================

pub fn listar_professores_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<Professor>, i32)> {
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Professor WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Professor WHERE id_escola = ?1 
         AND (nome_professor LIKE ?2 OR cpf LIKE ?2 OR email LIKE ?2)"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT id_professor, id_escola, nome_professor, nome_completo, email, telefone, 
                endereco, data_nascimento, cpf, data_contratacao, salario_atual 
         FROM Professor 
         WHERE id_escola = ?1 
         ORDER BY nome_professor 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT id_professor, id_escola, nome_professor, nome_completo, email, telefone, 
                endereco, data_nascimento, cpf, data_contratacao, salario_atual 
         FROM Professor 
         WHERE id_escola = ?1 
         AND (nome_professor LIKE ?4 OR cpf LIKE ?4 OR email LIKE ?4)
         ORDER BY nome_professor 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let professores = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(Professor {
                id_professor: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_professor: row.get(2)?,
                nome_completo: row.get(3)?,
                email: row.get(4)?,
                telefone: row.get(5)?,
                endereco: row.get(6)?,
                data_nascimento: row.get(7)?,
                cpf: row.get(8)?,
                data_contratacao: row.get(9)?,
                salario_atual: row.get(10)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(Professor {
                id_professor: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_professor: row.get(2)?,
                nome_completo: row.get(3)?,
                email: row.get(4)?,
                telefone: row.get(5)?,
                endereco: row.get(6)?,
                data_nascimento: row.get(7)?,
                cpf: row.get(8)?,
                data_contratacao: row.get(9)?,
                salario_atual: row.get(10)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((professores, total))
}

// ==================================================
// PAGINAÇÃO + FILTROS - TURMAS
// ==================================================

pub fn listar_turmas_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<TurmaInfo>, i32)> {
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Turma WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Turma t 
         JOIN Curso c ON t.id_curso = c.id_curso
         WHERE t.id_escola = ?1 
         AND (t.nome_turma LIKE ?2 OR c.nome_curso LIKE ?2)"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT t.id_turma, t.nome_turma, t.id_curso, c.nome_curso, t.ano, t.semestre 
         FROM Turma t
         JOIN Curso c ON t.id_curso = c.id_curso
         WHERE t.id_escola = ?1 
         ORDER BY t.nome_turma 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT t.id_turma, t.nome_turma, t.id_curso, c.nome_curso, t.ano, t.semestre 
         FROM Turma t
         JOIN Curso c ON t.id_curso = c.id_curso
         WHERE t.id_escola = ?1 
         AND (t.nome_turma LIKE ?4 OR c.nome_curso LIKE ?4)
         ORDER BY t.nome_turma 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let turmas = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(TurmaInfo {
                id_turma: row.get(0)?,
                nome_turma: row.get(1)?,
                id_curso: row.get(2)?,
                nome_curso: row.get(3)?,
                ano: row.get(4)?,
                semestre: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(TurmaInfo {
                id_turma: row.get(0)?,
                nome_turma: row.get(1)?,
                id_curso: row.get(2)?,
                nome_curso: row.get(3)?,
                ano: row.get(4)?,
                semestre: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((turmas, total))
}

// ==================================================
// PAGINAÇÃO + FILTROS - CURSOS
// ==================================================

pub fn listar_cursos_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<Curso>, i32)> {
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Curso WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Curso WHERE id_escola = ?1 
         AND nome_curso LIKE ?2"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT id_curso, id_escola, nome_curso, descricao, duracao_horas 
         FROM Curso 
         WHERE id_escola = ?1 
         ORDER BY nome_curso 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT id_curso, id_escola, nome_curso, descricao, duracao_horas 
         FROM Curso 
         WHERE id_escola = ?1 
         AND nome_curso LIKE ?4
         ORDER BY nome_curso 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let cursos = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(Curso {
                id_curso: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_curso: row.get(2)?,
                descricao: row.get(3)?,
                duracao_horas: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(Curso {
                id_curso: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_curso: row.get(2)?,
                descricao: row.get(3)?,
                duracao_horas: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((cursos, total))
}

// ==================================================
// PAGINAÇÃO + FILTROS - DISCIPLINAS
// ==================================================

pub fn listar_disciplinas_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<Disciplina>, i32)> {
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Disciplina WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Disciplina WHERE id_escola = ?1 
         AND nome_disciplina LIKE ?2"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT id_disciplina, id_escola, nome_disciplina, descricao, carga_horaria 
         FROM Disciplina 
         WHERE id_escola = ?1 
         ORDER BY nome_disciplina 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT id_disciplina, id_escola, nome_disciplina, descricao, carga_horaria 
         FROM Disciplina 
         WHERE id_escola = ?1 
         AND nome_disciplina LIKE ?4
         ORDER BY nome_disciplina 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let disciplinas = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(Disciplina {
                id_disciplina: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_disciplina: row.get(2)?,
                descricao: row.get(3)?,
                carga_horaria: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(Disciplina {
                id_disciplina: Some(row.get(0)?),
                id_escola: row.get(1)?,
                nome_disciplina: row.get(2)?,
                descricao: row.get(3)?,
                carga_horaria: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((disciplinas, total))
}

// ==================================================
// PAGINAÇÃO + FILTROS - MATRÍCULAS
// ==================================================

pub fn listar_matriculas_paginado(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro: &str,
) -> Result<(Vec<MatriculaInfo>, i32)> {
    let count_query = if filtro.is_empty() {
        "SELECT COUNT(*) FROM Matricula WHERE id_escola = ?1"
    } else {
        "SELECT COUNT(*) FROM Matricula m
         JOIN Aluno a ON m.id_aluno = a.id_aluno
         JOIN Turma t ON m.id_turma = t.id_turma
         WHERE m.id_escola = ?1 
         AND (a.nome_aluno LIKE ?2 OR t.nome_turma LIKE ?2)"
    };
    
    let total: i32 = if filtro.is_empty() {
        conn.query_row(count_query, params![id_escola], |row| row.get(0))?
    } else {
        let filtro_query = format!("%{}%", filtro);
        conn.query_row(count_query, params![id_escola, filtro_query], |row| row.get(0))?
    };
    
    let offset = (pagina - 1) * por_pagina;
    
    let data_query = if filtro.is_empty() {
        "SELECT m.id_matricula, a.nome_aluno, t.nome_turma, m.data_matricula, m.status 
         FROM Matricula m
         JOIN Aluno a ON m.id_aluno = a.id_aluno
         JOIN Turma t ON m.id_turma = t.id_turma
         WHERE m.id_escola = ?1 
         ORDER BY a.nome_aluno 
         LIMIT ?2 OFFSET ?3"
    } else {
        "SELECT m.id_matricula, a.nome_aluno, t.nome_turma, m.data_matricula, m.status 
         FROM Matricula m
         JOIN Aluno a ON m.id_aluno = a.id_aluno
         JOIN Turma t ON m.id_turma = t.id_turma
         WHERE m.id_escola = ?1 
         AND (a.nome_aluno LIKE ?4 OR t.nome_turma LIKE ?4)
         ORDER BY a.nome_aluno 
         LIMIT ?2 OFFSET ?3"
    };
    
    let mut stmt = conn.prepare(data_query)?;
    
    let matriculas = if filtro.is_empty() {
        stmt.query_map(params![id_escola, por_pagina, offset], |row| {
            Ok(MatriculaInfo {
                id_matricula: row.get(0)?,
                nome_aluno: row.get(1)?,
                nome_turma: row.get(2)?,
                data_matricula: row.get(3)?,
                status: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        let filtro_query = format!("%{}%", filtro);
        stmt.query_map(params![id_escola, por_pagina, offset, filtro_query], |row| {
            Ok(MatriculaInfo {
                id_matricula: row.get(0)?,
                nome_aluno: row.get(1)?,
                nome_turma: row.get(2)?,
                data_matricula: row.get(3)?,
                status: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    Ok((matriculas, total))
}