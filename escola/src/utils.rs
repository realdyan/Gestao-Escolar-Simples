use std::io::{self, Write};

// Função utilitária para ler entrada do usuário com um prompt
pub fn ler_entrada(label: &str) -> String {
    print!("{}: ", label);
    // O flush é necessário porque o print! (sem \n) não sai na hora no terminal
    io::stdout().flush().unwrap(); 

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha do teclado");

    input.trim().to_string()
}

/// Lê a entrada e tenta converter para um número inteiro (i32)
pub fn ler_inteiro(label: &str) -> i32 {
    loop {
        let input = ler_entrada(label); // Reutiliza sua função de string

        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, digite um número válido."),
        }
    }
}

/// Função utilitária para pausar o programa (útil após um cadastro)
pub fn esperar_enter() {
    println!("\nPressione Enter para continuar...");
    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).unwrap();
}

// Função utilitária para ler entrada do usuário e converter para f64(float)
pub fn ler_float(label: &str) -> f64 {
    loop {
        let input = ler_entrada(label);
        match input.replace(',', ".").parse::<f64>() { // replace ajuda se o usuário usar vírgula
            Ok(num) => return num,
            Err(_) => println!("Por favor, digite um valor numérico válido (Ex: 1500.50)."),
        }
    }
}