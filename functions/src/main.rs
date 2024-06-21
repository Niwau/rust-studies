/**
 * 1. Funções são declaradas com a palavra-chave `fn`
 * 2. O tipo de retorno de uma função deve ser anotado
 * 3. Em Rust, o valor de retorno de uma função é a última expressão
 * 4. Não pode haver ponto e vírgula no final da última expressão se for o valor de retorno
 */

fn main() {
    // Função sem parâmetros e sem retorno
    fn hello_world() {
        println!("Hello, world!");
    }

    hello_world();

    // Função com parâmetros e sem retorno
    fn print_number(x: i32) {
        println!("O número é: {}", x);
    }

    print_number(10);

    // Função sem parâmetros e com retorno
    fn get_number() -> i32 {
        10
    }

    let number = get_number();
    println!("O número é: {}", number);

    // Função com parâmetros e com retorno
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
