/**
 * 1. A diferença entre constantes e variáveis é que constantes nunca podem ser mutadas
 * 2. Constantes são declaradas com a palavra-chave `const`
 * 3. O tipo de uma constante deve ser anotado
 * 4. Também existe o tipo `static`, que pode ser alterada com a palavra-chave `mut`
 */

fn main() {
    const PI: f32 = 3.14;
    println!("O valor de PI é: {}", PI);

    static LANGUAGE: &str = "Rust";
    println!("A linguagem é: {}", LANGUAGE);
}
