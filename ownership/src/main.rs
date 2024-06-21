/*
    1. Ownership é um sistema de gerenciamento de memória que garante que o código não tenha memory leaks ou double frees.
    2. Ownership é uma das características mais importantes do Rust.
    3. Ownership é baseado em três regras:
        - Cada valor em Rust tem uma variável que é chamada de Owner do valor.
        - Um valor só pode ter um dono por vez.
        - Quando o dono sai do escopo, o valor é descartado.
    4. No entanto, alguns tipos de dados são copiados por padrão em vez de serem movidos.
    5. O conceito de Ownership também se aplica a funções.
    6. O Owner é parecido com um ponteiro, mas com algumas regras adicionais.
    7. Também temos o conceito de Borrowing, que é uma forma de emprestar um valor sem transferir a propriedade.
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

    // Funções
    let s5 = String::from("hello");

    fn owner(t: String) {
        println!("{}", t);
    }

    owner(s5); // s5 foi movido para a função owner, portanto, s5 não é mais válido
    // println!("{}", s5); // s5 não é mais válido

    // Retornando Ownership

    let s5 = String::from("hello");

    fn owner2(mut a: String) -> String {
        a.push_str(" world");
        a
    }

    let s6 = owner2(s5);

    // s5 não é mais válido porque o Owner foi transferido para s6

    println!("{}", s6);

    // Borrowing
    // Estamos passando uma referência para a função owner3 em vez de passar o valor, portanto, s7 ainda é válido
    let s7 = String::from("hello");

    fn owner3(a: &String) {
        println!("{}", a);
    }

    owner3(&s7); // s7 foi emprestado para a função owner3, portanto, s7 ainda é válido



}
