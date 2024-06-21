/*
    1. Ownership é um sistema de gerenciamento de memória que garante que o código não tenha memory leaks ou double frees.
    2. Ownership é uma das características mais importantes do Rust.
    3. Ownership é baseado em três regras:
        - Cada valor em Rust tem uma variável que é chamada de Owner do valor.
        - Um valor só pode ter um dono por vez.
        - Quando o dono sai do escopo, o valor é descartado.
    4. No entanto, alguns tipos de dados são copiados por padrão em vez de serem movidos.
*/

fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); // O Owner de s1 foi transferido para s2, então s1 não é mais válido
    println!("{}", s2); // s2 é válido

    // Clonando
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // Exceções (Inteiros, Floats, Booleans e Caracteres)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    
}
