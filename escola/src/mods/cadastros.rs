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

// ========================================================
// ================== MENUS DE CADASTROS ==================

// -- Função Exibir Menu de Cadastros Principal --
pub fn exibir_menu_cadastros(conn: &Connection) {
    loop {
        println!("\n--- Menu de Cadastros ---");
        println!("1 - Cadastrar Escola");
        println!("2 - Cadastrar Professor");
        println!("3 - Cadastrar Curso");
        println!("7 - Cadastrar Aluno");
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

            3 => println!("Opção 3 em desenvolvimento..."),
            7 => println!("Opção Aluno em desenvolvimento..."),
            0 => break,
            _ => println!("Opção inválida!"),
        }
    }
}
