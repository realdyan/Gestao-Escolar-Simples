
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


// -- Função Exibir Menu de Cadastros Principal --
pub fn exibir_menu_cadastros() {
    print!("\n");
    println!("--- Menu de Cadastros ---");
    println!("1 - Cadastrar Professor");
    println!("2 - Cadastrar Curso");
    println!("3 - Cadastrar Nível");
    println!("4 - Cadastrar Turma");
    println!("5 - Cadastrar Disciplina");
    println!("6 - Cadastrar Aluno");
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
        1 => println!("Opção escolhida: Cadastrar Professor"),
        2 => println!("Opção escolhida: Cadastrar Curso"),
        3 => println!("Opção escolhida: Cadastrar Nível"),
        4 => println!("Opção escolhida: Cadastrar Turma"),
        5 => println!("Opção escolhida: Cadastrar Disciplina"),
        6 => println!("Opção escolhida: Cadastrar Aluno"),
        0 => println!("Voltando ao Menu Principal..."),
        _ => println!("Opção inválida!"),
    }


}