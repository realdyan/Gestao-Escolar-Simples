use crate::utils;
use rusqlite::Connection;

pub struct Matricula {
    pub id_matricula: Option<i32>,
    pub id_aluno: i32,
    pub id_turma: i32,
    pub data_matricula: String,
    pub status: String,
}

pub fn exibir_menu_matriculas(conn: &Connection) {
    loop {
        println!("\n--- Gerenciamento de Matrículas ---");
        println!("1 - Realizar Nova Matrícula");
        println!("2 - Listar Matrículas");
        println!("3 - Cancelar/Alterar Status");
        println!("0 - Voltar");

        let opcao = utils::ler_inteiro("Escolha uma opção");

        match opcao {
            1 => {
                println!("\n--- Nova Matrícula ---");
                let id_aluno = utils::ler_inteiro("ID do Aluno");
                let id_turma = utils::ler_inteiro("ID da Turma");
                let data = utils::ler_entrada("Data (DD/MM/AAAA)");
                
                let m = Matricula {
                    id_matricula: None,
                    id_aluno,
                    id_turma,
                    data_matricula: data,
                    status: String::from("Ativo"),
                };

                match crate::db::inserir_matricula(conn, &m) {
                    Ok(_) => println!("Sucesso: Aluno matriculado!"),
                    Err(e) => println!("Erro: Verifique se o ID do Aluno e da Turma existem. ({})", e),
                }
                utils::esperar_enter();
            }
            2 => {
                if let Ok(matriculas) = crate::db::listar_matriculas_db(conn) {
                    for m in matriculas {
                        println!("Matrícula ID: {} | Aluno ID: {} | Turma ID: {} | Status: {}", 
                            m.id_matricula.unwrap_or(0), m.id_aluno, m.id_turma, m.status);
                    }
                }
                utils::esperar_enter();
            }
            0 => break,
            _ => println!("Opção inválida!"),
        }
    }
}