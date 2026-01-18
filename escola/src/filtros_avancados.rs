use rusqlite::Connection;
use crate::db;

// ==================================================
// BUILDER DE QUERIES DINÂMICAS (OTIMIZADO)
// ==================================================

pub struct QueryBuilder {
    base_query: String,
    where_clauses: Vec<String>,
    params: Vec<rusqlite::types::Value>,
    joins: Vec<String>,
}

impl QueryBuilder {
    pub fn new(base_query: &str) -> Self {
        Self {
            base_query: base_query.to_string(),
            where_clauses: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, clause: &str, value: rusqlite::types::Value) {
        self.where_clauses.push(clause.to_string());
        self.params.push(value);
    }

    pub fn add_text_filter(&mut self, field: &str, value: &str) {
        if !value.trim().is_empty() {
            self.where_clauses.push(format!("{} LIKE ?", field));
            self.params.push(rusqlite::types::Value::Text(format!("%{}%", value)));
        }
    }

    pub fn add_date_range(&mut self, field: &str, inicio: &str, fim: &str) {
        if !inicio.trim().is_empty() {
            self.where_clauses.push(format!("date({}) >= date(?)", field));
            self.params.push(rusqlite::types::Value::Text(inicio.to_string()));
        }
        if !fim.trim().is_empty() {
            self.where_clauses.push(format!("date({}) <= date(?)", field));
            self.params.push(rusqlite::types::Value::Text(fim.to_string()));
        }
    }

    pub fn build(&self) -> (String, Vec<rusqlite::types::Value>) {
        let mut query = self.base_query.clone();
        
        if !self.joins.is_empty() {
            for join in &self.joins {
                query.push_str(" ");
                query.push_str(join);
            }
        }
        
        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.where_clauses.join(" AND "));
        }
        
        (query, self.params.clone())
    }

    pub fn build_count(&self) -> (String, Vec<rusqlite::types::Value>) {
        let mut query = "SELECT COUNT(*) FROM ".to_string();
        
        if let Some(from_index) = self.base_query.to_uppercase().find("FROM") {
            let from_part = &self.base_query[from_index + 4..];
            query.push_str(from_part.trim());
        }
        
        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.where_clauses.join(" AND "));
        }
        
        (query, self.params.clone())
    }
}

// ==================================================
// FUNÇÕES DE FILTRO (INTEGRADAS COM O BUILDER)
// ==================================================

pub fn filtrar_alunos(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro_nome: &str,
    filtro_cpf: &str,
    filtro_email: &str,
    filtro_telefone: &str,
    filtro_data_inicio: &str,
    filtro_data_fim: &str,
) -> Result<(Vec<db::Aluno>, i32), rusqlite::Error> {
    
    let mut builder = QueryBuilder::new(
        "SELECT id_aluno, id_escola, nome_aluno, nome_completo, email, telefone, 
         endereco, data_nascimento, cpf, data_matricula FROM Aluno"
    );
    
    // 1. Filtro base obrigatório
    builder.add_filter("id_escola = ?", rusqlite::types::Value::Integer(id_escola as i64));
    
    // 2. Filtros dinâmicos
    builder.add_text_filter("nome_aluno", filtro_nome);
    builder.add_text_filter("cpf", filtro_cpf);
    builder.add_text_filter("email", filtro_email);
    builder.add_text_filter("telefone", filtro_telefone);
    builder.add_date_range("data_matricula", filtro_data_inicio, filtro_data_fim);
    
    // 3. Contagem Total (para paginação na UI)
    let (count_sql, count_params) = builder.build_count();
    let total: i32 = conn.query_row(
        &count_sql, 
        rusqlite::params_from_iter(count_params.iter()), 
        |row| row.get(0)
    )?;
    
    // 4. Busca de Dados Paginados
    let (data_sql, mut data_params) = builder.build();
    let offset = (pagina - 1) * por_pagina;
    let paginated_sql = format!("{} ORDER BY nome_aluno ASC LIMIT ? OFFSET ?", data_sql);
    
    data_params.push(rusqlite::types::Value::Integer(por_pagina as i64));
    data_params.push(rusqlite::types::Value::Integer(offset as i64));
    
    let mut stmt = conn.prepare(&paginated_sql)?;
    let alunos_iter = stmt.query_map(rusqlite::params_from_iter(data_params.iter()), |row| {
        Ok(db::Aluno {
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
    })?;
    
    let mut alunos = Vec::new();
    for aluno in alunos_iter {
        alunos.push(aluno?);
    }
    
    Ok((alunos, total))
}

pub fn filtrar_professores(
    conn: &Connection,
    id_escola: i32,
    pagina: i32,
    por_pagina: i32,
    filtro_nome: &str,
    filtro_cpf: &str,
    filtro_email: &str,
    filtro_data_inicio: &str,
    filtro_data_fim: &str,
) -> Result<(Vec<db::Professor>, i32), rusqlite::Error> {
    
    let mut builder = QueryBuilder::new(
        "SELECT id_professor, id_escola, nome_professor, nome_completo, email, 
         telefone, endereco, data_nascimento, cpf, data_contratacao, salario_atual FROM Professor"
    );
    
    builder.add_filter("id_escola = ?", rusqlite::types::Value::Integer(id_escola as i64));
    builder.add_text_filter("nome_professor", filtro_nome);
    builder.add_text_filter("cpf", filtro_cpf);
    builder.add_text_filter("email", filtro_email);
    builder.add_date_range("data_contratacao", filtro_data_inicio, filtro_data_fim);
    
    let (count_sql, count_params) = builder.build_count();
    let total: i32 = conn.query_row(
        &count_sql, 
        rusqlite::params_from_iter(count_params.iter()), 
        |row| row.get(0)
    )?;
    
    let (data_sql, mut data_params) = builder.build();
    let offset = (pagina - 1) * por_pagina;
    let paginated_sql = format!("{} ORDER BY nome_professor ASC LIMIT ? OFFSET ?", data_sql);
    
    data_params.push(rusqlite::types::Value::Integer(por_pagina as i64));
    data_params.push(rusqlite::types::Value::Integer(offset as i64));
    
    let mut stmt = conn.prepare(&paginated_sql)?;
    let profs_iter = stmt.query_map(rusqlite::params_from_iter(data_params.iter()), |row| {
        Ok(db::Professor {
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
    })?;
    
    let mut professores = Vec::new();
    for prof in profs_iter {
        professores.push(prof?);
    }
    
    Ok((professores, total))
}