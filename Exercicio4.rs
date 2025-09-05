fn calcular_media(n1: f64, n2: f64, n3: f64) -> f64 {
    (n1 * 2.0 + n2 * 3.0 + n3 * 5.0) / 10.0
}

fn main() {
    let nota1 = 7.5;
    let nota2 = 8.0;
    let nota3 = 9.2;

    let media = calcular_media(nota1, nota2, nota3);

    println!("Média: {:.2}", media);

    if media >= 7.0 {
        println!("Aprovado!");
    } else {
        println!("Reprovado!");
    }
}