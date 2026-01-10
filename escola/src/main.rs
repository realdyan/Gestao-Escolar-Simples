// ============= Importações =============
use std::io;


use crate::mods::cadastros;
use crate::db;
use crate::utils;


// ============= Módulos =============
pub mod mods;
/*
pub mod cadastros; 
pub mod matriculas;
pub mod alunos;
pub mod professores;
pub mod relatorios;
pub mod pagamentos;
pub mod outras_opcoes;
*/



// ============= Função Principal (MAIN) =============
fn main() {
    println!("Bem vindo a Escola X!");    
    println!("\n");

    // -- Menu Principal --
    loop { // Loop do Menu Principal
    println!("Menu de Opções:");
    println!("Escolha uma das opções abaixo:");
    println!("1 - Cadastros");
    println!("2 - Matriculas");
    println!("3 - Alunos");
    println!("4 - Professores");
    println!("5 - Relatórios");
    println!("6 - Pagamentos");
    println!("7 - Outras Opções");
    println!("0 - Sair");
    println!("\n");

    // Variável para armazenar a opção escolhida
    let mut opcao = String::new();

    // Lê a opção escolhida pelo usuário
    io::stdin()
    .read_line(&mut opcao)
    .expect("Falha ao ler a opção escolhida");

    // Converte a opção para número inteiro
    let opcao: u32 = opcao
    .trim()
    .parse()
    .expect("Por favor, digite um número válido");

    print!("\n");
    match opcao {
        // 1. Cadastros
        1 => {
            loop { // Loop do Menu de Cadastros
                cadastros::exibir_menu_cadastros();
                break;
            }
        }
        // 2. Matriculas
        2 => println!("2 - Menu de Matriculas"),
        3 => println!("3 - Menu de Alunos"),
        4 => println!("4 - Menu de Professores"),
        5 => println!("5 - Menu de Relatórios"),
        6 => println!("6 - Menu de Pagamentos"),
        7 => println!("7 - Menu de Outras Opções"),
        0 => {
            println!("Saindo...");
            break; // Sai do loop principal
        }
        _ => println!("Opção inválida!"),
    }
    println!("\n");
}

    println!("Encerrando o sistema. Até logo!");

}
