use std::io;

fn verificar_senha(senha: &str) -> bool {
    let tem_numero = senha.chars().any(|c| c.is_digit(10));
    let tem_maiuscula = senha.chars().any(|c| c.is_ascii_uppercase());
    let tamanho_ok = senha.len() >= 8;

    tem_numero && tem_maiuscula && tamanho_ok
}

fn main() {
    loop {
        let mut senha = String::new();
        println!("Digite a senha: ");
        io::stdin().read_line(&mut senha).unwrap();
        let senha = senha.trim();

        if verificar_senha(senha) {
            println!("Senha válida! Acesso concedido.");
            break;
        } else {
            println!("Senha inválida! Tente novamente.");
        }
    }
}