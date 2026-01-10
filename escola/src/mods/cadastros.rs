use rusqlite::Connection;

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
    let mut nome_escola = String::new();
    let mut email = String::new();
    let mut telefone = String::new();
    let mut endereco = String::new();
    let mut website = String::new();

    println!("--- Cadastro de Escola ---");

    println!("Digite o nome da escola:");
    std::io::stdin()
        .read_line(&mut nome_escola)
        .expect("Falha ao ler o nome da escola");
    let nome_escola = nome_escola.trim().to_string();

    println!("Digite o email da escola:");
    std::io::stdin()
        .read_line(&mut email)
        .expect("Falha ao ler o email da escola");
    let email = email.trim().to_string();

    println!("Digite o telefone da escola:");
    std::io::stdin()
        .read_line(&mut telefone)
        .expect("Falha ao ler o telefone da escola");
    let telefone = telefone.trim().to_string();

    println!("Digite o endereço da escola:");
    std::io::stdin()
        .read_line(&mut endereco)
        .expect("Falha ao ler o endereço da escola");
    let endereco = endereco.trim().to_string();

    println!("Digite o website da escola:");
    std::io::stdin()
        .read_line(&mut website)
        .expect("Falha ao ler o website da escola");
    let website = website.trim().to_string();

    nova_escola(nome_escola, email, telefone, endereco, website)
} 

// -- Função Exibir Menu de Cadastros Principal --
pub fn exibir_menu_cadastros(conn: &Connection) {
    print!("\n");
    println!("--- Menu de Cadastros ---");
    println!("1 - Cadastrar Escola");
    println!("2 - Cadastrar Professor");
    println!("3 - Cadastrar Curso");
    println!("4 - Cadastrar Nível");
    println!("5 - Cadastrar Turma");
    println!("6 - Cadastrar Disciplina");
    println!("7 - Cadastrar Aluno");
    println!("0 - Voltar ao Menu Principal");

    let mut opcao = String::new();

    std::io::stdin()
    .read_line(&mut opcao)
    .expect("Falha ao ler a opção escolhida");

    let opcao: u32 = opcao
    .trim()
    .parse()
    .expect("Por favor, digite um número válido");
    
    print!("\n");
    match opcao {
        1 => {
            loop { // Loop do Menu de Cadastro de Escola
            println!("\n");
            println!("Opção escolhida: Cadastro de Escola");
            println!("1 - Cadastrar nova Escola");
            println!("2 - Atualizar dados da Escola");
            println!("3 - Listar Escolas cadastradas");
            println!("4 - Remover Escola");
            println!("0 - Voltar ao Menu de Cadastro de Escola");
            
            let mut sub_opcao = String::new();
            std::io::stdin()
            .read_line(&mut sub_opcao)
            .expect("Falha ao ler a opção escolhida");

            let sub_opcao: u32 = sub_opcao
            .trim()
            .parse()
            .expect("Por favor, digite um número válido");
           
            print!("\n");

            match sub_opcao {
                // Cadastrar nova escola
                1 => {
                    println!("Cadastrando nova escola.");
                    let escola_criada = cadastrar_escola();
                    match crate::db::inserir_escola(conn, &escola_criada) {
                        Ok(_) => println!("Escola cadastrada com sucesso no banco de dados!"),
                        Err(e) => println!("Erro ao cadastrar escola no banco de dados: {}", e),
                    }
                }
                // Atualizar dados da escola
                2 => {
                    println!("Atualizando dados da escola...");

                    println!("Digite o ID da escola que deseja atualizar:");
                    let mut id_str = String::new();
                    std::io::stdin().read_line(&mut id_str).expect("Erro ao ler ID");
                    let id: i32 = id_str.trim().parse().expect("Digite um número válido");
               
                    println!("Agora, insira os NOVOS dados:");
                    let escola_atualizada = cadastrar_escola();
                    match crate::db::atualizar_escola_db(conn, id, &escola_atualizada) {
                        Ok(_) => println!("Escola atualizada com sucesso no banco de dados!"),
                        Err(e) => println!("Erro ao atualizar escola no banco de dados: {}", e),
                    }
                }
                // Listar escolas cadastradas
                3 => {
                    println!("Listando escolas cadastradas...");
                    match crate::db::listar_escolas_db(conn) {
                        Ok(escolas) => {
                            for escola in escolas {
                                exibir_escola(&escola);
                                println!("-----------------------");
                            }
                        }
                        Err(e) => println!("Erro ao listar escolas do banco de dados: {}", e),
                    }
                }
                // Remover escola
                4 => {
                    println!("Removendo escola...");
                    println!("Digite o ID da escola que deseja remover:");
                    let mut id_str = String::new();
                    std::io::stdin().read_line(&mut id_str).expect("Erro ao ler ID");
                    let id: i32 = id_str.trim().parse().expect("Digite um número válido");

                    match crate::db::remover_escola_db(conn, id) {
                        Ok(_) => println!("Escola removida com sucesso do banco de dados!"),
                        Err(e) => println!("Erro ao remover escola do banco de dados: {}", e),
                    }
                }  
                // Voltar ao Menu de Cadastros
                0 => {
                    println!("Voltando ao Menu de Cadastros...");
                    break; // Sai do loop do Menu de Cadastro de Escola
                }
                _ => println!("Opção inválida!"),
            }
        }
    }

        2 => println!("Opção escolhida: Cadastro de Professor"),
        3 => println!("Opção escolhida: Cadastro de Curso"),
        4 => println!("Opção escolhida: Cadastro de Nível"),
        5 => println!("Opção escolhida: Cadastro de Turma"),
        6 => println!("Opção escolhida: Cadastro de Disciplina"),
        7 => println!("Opção escolhida: Cadastro de Aluno"),
        0 => println!("Voltando ao Menu Principal..."),
        _ => println!("Opção inválida!"),
    }

}

