use crate::utils;
use rusqlite::Connection;

// ===================================================
// ===================== ESCOLA  =====================

// Representação de uma Escola no Rust
pub struct Escola {
    pub id_escola: Option<i32>,
    pub nome_escola: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub website: String,
}

// -- Função Implementação Escola (Inserção de dados pelo usuario)
pub fn nova_escola(
    nome_escola: String,
    email: String,
    telefone: String,
    endereco: String,
    website: String,
) -> Escola {
    Escola {
        id_escola: None,
        nome_escola,
        email,
        telefone,
        endereco,
        website,
    }
}

// -- Função Implementação Escola (Exibição de dados) --
pub fn exibir_escola(escola: &Escola) {
    println!("\n");
    println!("--- Dados da Escola ---");
    println!("ID: {:?}", escola.id_escola);
    println!("Nome: {}", escola.nome_escola);
    println!("Email: {}", escola.email);
    println!("Telefone: {}", escola.telefone);
    println!("Endereço: {}", escola.endereco);
    println!("Website: {}", escola.website);
}

// -- Função Implementação Escola (Atualização de dados pelo usuario)
pub fn atualizar_escola(
    escola: &mut Escola,
    nome_escola: String,
    email: String,
    telefone: String,
    endereco: String,
    website: String,
) {
    escola.nome_escola = nome_escola;
    escola.email = email;
    escola.telefone = telefone;
    escola.endereco = endereco;
    escola.website = website;
}

// -- Função Implementação Escola (Remoção de dados) --
pub fn remover_escola(escola: &mut Option<Escola>) {
    *escola = None;
}

// -- Função Cadasrtrar Escola pelo Usuário --
pub fn cadastrar_escola() -> Escola {
    println!("--- Cadastro de Escola ---");

    // O utils::ler_entrada já imprime a mensagem, lê o teclado e faz o trim
    let nome = utils::ler_entrada("Digite o nome da escola");
    let email = utils::ler_entrada("Digite o email da escola");
    let telefone = utils::ler_entrada("Digite o telefone da escola");
    let endereco = utils::ler_entrada("Digite o endereço da escola");
    let website = utils::ler_entrada("Digite o website da escola");

    // Retorna a struct limpa
    nova_escola(nome, email, telefone, endereco, website)
}

// ========================================================
// ===================== PROFESSORES  =====================

// Representação de um Professor no Rust
pub struct Professor {
    pub id_professor: Option<i32>,
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

// -- Função para inserir um novo professor --
pub fn novo_professor(
    nome_professor: String,
    nome_completo: String,
    email: String,
    telefone: String,
    endereco: String,
    data_nascimento: String,
    cpf: String,
    data_contratacao: String,
    salario_atual: f64,
) -> Professor {
    Professor {
        id_professor: None,
        nome_professor,
        nome_completo,
        email,
        telefone,
        endereco,
        data_nascimento,
        cpf,
        data_contratacao,
        salario_atual,
    }
}

pub fn cadastrar_professor() -> Professor {
    println!("\n--- Cadastro de Professor ---");

    let nome = utils::ler_entrada("Nome Curto (Ex: Claudio)");
    let nome_completo = utils::ler_entrada("Nome Completo");
    let email = utils::ler_entrada("Email");
    let telefone = utils::ler_entrada("Telefone");
    let endereco = utils::ler_entrada("Endereço");
    let data_nasc = utils::ler_entrada("Data de Nascimento (DD/MM/AAAA)");
    let cpf = utils::ler_entrada("CPF");
    let data_cont = utils::ler_entrada("Data de Contratação");

    // Como prometido, aqui está o uso do ler_float que vamos criar
    let salario = utils::ler_float("Salário Atual");

    novo_professor(
        nome,
        nome_completo,
        email,
        telefone,
        endereco,
        data_nasc,
        cpf,
        data_cont,
        salario,
    )
}

// =======================================================
// ======================= ALUNOS  =======================
pub struct Aluno {
    pub id_aluno: Option<i32>,
    pub nome_aluno: String,
    pub nome_completo: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub data_nascimento: String,
    pub cpf: String,
    pub data_matricula: String,
}

pub fn novo_aluno(
    nome_aluno: String,
    nome_completo: String,
    email: String,
    telefone: String,
    endereco: String,
    data_nascimento: String,
    cpf: String,
    data_matricula: String,
) -> Aluno {
    Aluno {
        id_aluno: None,
        nome_aluno,
        nome_completo,
        email,
        telefone,
        endereco,
        data_nascimento,
        cpf,
        data_matricula,
    }
}

// -- Interface de entrada de dados PELO USUARIO --
pub fn cadastrar_aluno() -> Aluno {
    println!("\n--- Cadastro / Edição de Aluno ---");
    let nome = utils::ler_entrada("Nome Curto");
    let nome_completo = utils::ler_entrada("Nome Completo");
    let email = utils::ler_entrada("Email");
    let telefone = utils::ler_entrada("Telefone");
    let endereco = utils::ler_entrada("Endereço");
    let data_nasc = utils::ler_entrada("Data de Nascimento (DD/MM/AAAA)");
    let cpf = utils::ler_entrada("CPF");
    let data_mat = utils::ler_entrada("Data de Matrícula");

    novo_aluno(
        nome,
        nome_completo,
        email,
        telefone,
        endereco,
        data_nasc,
        cpf,
        data_mat,
    )
}

// -- Exibição formatada Alunos --
pub fn exibir_aluno(a: &Aluno) {
    println!(
        "ID: {:<3} | Nome: {:<15} | CPF: {:<12} | Matrícula: {}",
        a.id_aluno.unwrap_or(0),
        a.nome_aluno,
        a.cpf,
        a.data_matricula
    );
}

// =======================================================
// ======================= CURSOS  =======================

pub struct Curso {
    pub id_curso: Option<i32>,
    pub nome_curso: String,
    pub descricao: String,
    pub duracao_horas: i32,
}

pub fn cadastrar_curso() -> Curso {
    println!("\n--- Cadastro de Curso ---");
    let nome = utils::ler_entrada("Nome do Curso");
    let desc = utils::ler_entrada("Descrição");
    let horas = utils::ler_inteiro("Duração em Horas");

    Curso {
        id_curso: None,
        nome_curso: nome,
        descricao: desc,
        duracao_horas: horas,
    }
}

pub fn exibir_curso(c: &Curso) {
    println!(
        "ID: {:<3} | Nome: {:<20} | Horas: {:<5} | Desc: {}",
        c.id_curso.unwrap_or(0),
        c.nome_curso,
        c.duracao_horas,
        c.descricao
    );
}


// =======================================================
// ======================= TURMAS  =======================
// Representação de uma Turma no Rust
pub struct Turma {
    pub id_turma: Option<i32>,
    pub nome_turma: String,
    pub id_curso: i32,
    pub ano: i32,
    pub semestre: i32,
}

// -- Função Cadastrar Turma pelo Usuário --
pub fn cadastrar_turma() -> Turma {
    println!("\n--- Cadastro de Turma ---");
    let nome = utils::ler_entrada("Nome da Turma (Ex: Turma A)");
    let id_curso = utils::ler_inteiro("ID do Curso vinculado");
    let ano = utils::ler_inteiro("Ano (Ex: 2026)");
    let semestre = utils::ler_inteiro("Semestre (1 ou 2)");

    Turma {
        id_turma: None,
        nome_turma: nome,
        id_curso,
        ano,
        semestre,
    }
}

// -- Exibição formatada Turmas --
pub fn exibir_turma(t: &Turma) {
    println!(
        "ID: {:<3} | Nome: {:<15} | Curso ID: {:<3} | Período: {}/{}",
        t.id_turma.unwrap_or(0),
        t.nome_turma,
        t.id_curso,
        t.ano,
        t.semestre
    );
}


// =======================================================
// ===================== DISCIPLINAS =====================

pub struct Disciplina {
    pub id_disciplina: Option<i32>,
    pub nome_disciplina: String,
    pub descricao: String,
    pub carga_horaria: i32,
    pub id_nivel: i32, // Níveis fixos: 1, 2 ou 3
}

pub fn cadastrar_disciplina() -> Disciplina {
    println!("\n--- Cadastro de Disciplina ---");
    let nome = utils::ler_entrada("Nome da Disciplina");
    let desc = utils::ler_entrada("Descrição");
    let carga = utils::ler_inteiro("Carga Horária (horas)");
    
    println!("Níveis disponíveis: 1-Básico, 2-Intermediário, 3-Avançado");
    let nivel = utils::ler_inteiro("Escolha o Nível (1-3)");

    Disciplina {
        id_disciplina: None,
        nome_disciplina: nome,
        descricao: desc,
        carga_horaria: carga,
        id_nivel: nivel,
    }
}

pub fn exibir_disciplina(d: &Disciplina) {
    let nivel_str = match d.id_nivel {
        1 => "Básico",
        2 => "Intermediário",
        3 => "Avançado",
        _ => "Desconhecido",
    };

    println!(
        "ID: {:<3} | Nome: {:<15} | Carga: {:<3}h | Nível: {}",
        d.id_disciplina.unwrap_or(0),
        d.nome_disciplina,
        d.carga_horaria,
        nivel_str
    );
}


// ========================================================
// ================== MENUS DE CADASTROS ==================

// -- Função Exibir Menu de Cadastros Principal --
pub fn exibir_menu_cadastros(conn: &Connection) {
    loop {
        println!("\n--- Menu de Cadastros ---");
        println!("1 - Cadastrar Escola");
        println!("2 - Cadastrar Professor");
        println!("3 - Cadastrar Curso");
        println!("4 - Cadastrar Aluno");
        println!("0 - Voltar ao Menu Principal");

        let opcao = utils::ler_inteiro("Escolha uma opção");

        match opcao {
            1 => loop {
                println!("\n--- Opções: Escola ---");
                println!("1 - Cadastrar Nova Escola");
                println!("2 - Atualizar Dados da Escola");
                println!("3 - Listar Todas as Escolas");
                println!("4 - Remover Escola");
                println!("0 - Voltar ao Menu Anterior");

                let sub_opcao = utils::ler_inteiro("Escolha uma opção");

                match sub_opcao {
                    1 => {
                        let escola_criada = cadastrar_escola();
                        let _ = crate::db::inserir_escola(conn, &escola_criada);
                        utils::esperar_enter();
                    }
                    2 => {
                        let id = utils::ler_inteiro("ID da escola para atualizar");
                        let escola_atualizada = cadastrar_escola();
                        let _ = crate::db::atualizar_escola_db(conn, id, &escola_atualizada);
                        utils::esperar_enter();
                    }
                    3 => {
                        if let Ok(escolas) = crate::db::listar_escolas_db(conn) {
                            for e in escolas {
                                exibir_escola(&e);
                            }
                        }
                        utils::esperar_enter();
                    }
                    4 => {
                        println!("\n--- REMOVER ESCOLA ---");
                        let confirmacao = utils::ler_entrada(
                            "Digite 'SIM' para confirmar a exclusão ou qualquer outra tecla para cancelar",
                        );

                        if confirmacao.to_uppercase() == "SIM" {
                            let id = utils::ler_inteiro("Digite o ID da escola para REMOVER");
                            match crate::db::remover_escola_db(conn, id) {
                                Ok(_) => println!("Sucesso: Escola removida do banco de dados."),
                                Err(e) => println!("Erro ao remover: {}", e),
                            }
                        } else {
                            println!("Acao cancelada pelo usuario.");
                        }
                        utils::esperar_enter();
                    }
                    0 => break,
                    _ => println!("Opção inválida!"),
                }
            },

            2 => loop {
                println!("\n--- Opções: Professor ---");
                println!("1 - Cadastrar Novo Professor");
                println!("2 - Atualizar Dados do Professor");
                println!("3 - Listar Todos os Professores");
                println!("4 - Remover Professor");
                println!("0 - Voltar ao Menu Anterior");

                let sub_opcao = utils::ler_inteiro("Escolha uma operação");

                match sub_opcao {
                    1 => {
                        let prof = cadastrar_professor();
                        let _ = crate::db::inserir_professor(conn, &prof);
                        utils::esperar_enter();
                    }
                    2 => {
                        let id = utils::ler_inteiro("ID do professor");
                        let prof_at = cadastrar_professor();
                        let _ = crate::db::atualizar_professor_db(conn, id, &prof_at);
                        utils::esperar_enter();
                    }
                    3 => {
                        if let Ok(profs) = crate::db::listar_professores_db(conn) {
                            for p in profs {
                                println!(
                                    "ID: {:<3} | Nome: {:<15} | Salário: R$ {:.2}",
                                    p.id_professor.unwrap_or(0),
                                    p.nome_professor,
                                    p.salario_atual
                                );
                            }
                        }
                        utils::esperar_enter();
                    }
                    4 => {
                        println!("\n--- REMOVER PROFESSOR ---");
                        let confirmacao = utils::ler_entrada(
                            "Digite 'SIM' para confirmar a exclusão ou qualquer outra tecla para cancelar",
                        );

                        if confirmacao.to_uppercase() == "SIM" {
                            let id = utils::ler_inteiro("Digite o ID do professor para REMOVER");
                            match crate::db::remover_professor_db(conn, id) {
                                Ok(_) => println!("Sucesso: Professor removido do banco de dados."),
                                Err(e) => println!("Erro ao remover: {}", e),
                            }
                        } else {
                            println!("Acao cancelada pelo usuario.");
                        }
                        utils::esperar_enter();
                    }
                    0 => break,
                    _ => println!("Opção inválida!"),
                }
            },

            3 => loop {
                println!("\n--- Gerenciamento de Cursos ---");
                println!("1 - Cadastrar Novo Curso");
                println!("2 - Listar Todos os Cursos");
                println!("3 - Atualizar Dados de um Curso");
                println!("4 - Remover Curso");
                println!("0 - Voltar");

                let sub_opcao = utils::ler_inteiro("Escolha uma opção:");

                match sub_opcao {
                    1 => {
                        let curso = cadastrar_curso();
                        let _ = crate::db::inserir_curso(conn, &curso);
                        utils::esperar_enter();
                    }
                    2 => {
                        if let Ok(cursos) = crate::db::listar_cursos_db(conn) {
                            println!("\n--- Lista de Cursos ---");
                            for c in cursos {
                                exibir_curso(&c);
                            }
                        }
                        utils::esperar_enter();
                    }
                    3 => {
                        let id = utils::ler_inteiro("Digite o ID do curso para atualizar");
                        let curso_novo = cadastrar_curso();
                        match crate::db::atualizar_curso_db(conn, id, &curso_novo) {
                            Ok(_) => println!("Sucesso: Curso atualizado no banco de dados."),
                            Err(e) => println!("Erro ao atualizar: {}", e),
                        }
                        utils::esperar_enter();
                    }
                    4 => {
                        println!("\n--- REMOVER CURSO ---");
                        let confirmacao = utils::ler_entrada(
                            "Digite 'SIM' para confirmar a exclusão ou qualquer outra tecla para cancelar",
                        );

                        if confirmacao.to_uppercase() == "SIM" {
                            let id = utils::ler_inteiro("Digite o ID do curso para REMOVER");
                            match crate::db::remover_curso_db(conn, id) {
                                Ok(_) => println!("Sucesso: Curso removido do banco de dados."),
                                Err(e) => println!("Erro ao remover: {}", e),
                            }
                        } else {
                            println!("Açao cancelada pelo usuario.");
                        }
                        utils::esperar_enter();
                    }
                    0 => break,
                    _ => println!("Opção inválida!"),
                }


            },
            4 => loop {
                println!("\n--- Opções: Aluno ---");
                println!("1 - Cadastrar Novo Aluno");
                println!("2 - Atualizar Dados do Aluno");
                println!("3 - Listar Todos os Alunos");
                println!("4 - Remover Aluno");
                println!("0 - Voltar ao Menu Anterior");

                let sub_opcao = utils::ler_inteiro("Escolha uma opção:");

                match sub_opcao {
                    1 => {
                        let aluno = cadastrar_aluno();
                        let _ = crate::db::inserir_aluno(conn, &aluno);
                        utils::esperar_enter();
                    }
                    2 => {
                        let id = utils::ler_inteiro("ID do aluno");
                        let aluno_at = cadastrar_aluno();
                        let _ = crate::db::atualizar_aluno_db(conn, id, &aluno_at);
                        utils::esperar_enter();
                    }
                    3 => {
                        if let Ok(alunos) = crate::db::listar_alunos_db(conn) {
                            for a in alunos {
                                exibir_aluno(&a);
                            }
                        }
                        utils::esperar_enter();
                    }
                    4 => {
                        println!("\n--- REMOVER ALUNO ---");
                        let confirmacao = utils::ler_entrada(
                            "Digite 'SIM' para confirmar a exclusão ou qualquer outra tecla para cancelar",
                        );

                        if confirmacao.to_uppercase() == "SIM" {
                            let id = utils::ler_inteiro("Digite o ID do aluno para REMOVER");
                            match crate::db::remover_aluno_db(conn, id) {
                                Ok(_) => println!("Sucesso: Aluno removido do banco de dados."),
                                Err(e) => println!("Erro ao remover: {}", e),
                            }
                        } else {
                            println!("Açao cancelada pelo usuario.");
                        }
                        utils::esperar_enter();
                    }
                    0 => break, // Opção para sair do menu de Alunos
                    _ => println!("Opção inválida!"),
                }
            },
            0 => break,
            _ => println!("Opção inválida!"),
        }
    }
}
