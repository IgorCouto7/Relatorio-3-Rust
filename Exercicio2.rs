use std::io;

fn eh_par(numero: i32) -> bool {
    numero % 2 == 0
}

fn main() {
    let mut escolha = String::new();
    println!("Jogador 1, escolha (par/ímpar): ");
    io::stdin().read_line(&mut escolha).unwrap();
    let escolha = escolha.trim().to_lowercase();

    let mut entrada = String::new();
    println!("Jogador 1, digite um número: ");
    io::stdin().read_line(&mut entrada).unwrap();
    let num1: i32 = entrada.trim().parse().unwrap();

    entrada.clear();
    println!("Jogador 2, digite um número: ");
    io::stdin().read_line(&mut entrada).unwrap();
    let num2: i32 = entrada.trim().parse().unwrap();

    let soma = num1 + num2;
    println!("A soma foi: {}", soma);

    if (eh_par(soma) && escolha == "par") || (!eh_par(soma) && escolha == "ímpar") {
        println!("Jogador 1 venceu!");
    } else {
        println!("Jogador 2 venceu!");
    }
}