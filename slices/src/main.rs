/**
 * 1. Slices são uma referência a uma parte de um valor.
 * 2. Existem dois tipos de strings em Rust: String e &str.
 * 3. String é um tipo de dado que é alocado na heap e é mutável.
 * 4. &str é uma referência a uma parte de uma String e é imutável.
 * 5. Sempre que criamos uma string literal, o tipo é &str.
 */

fn main() {
    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];

    println!("{}", hello);

    fn first_word(s: &String) -> &str {
        &s[..]
    }

    let word = first_word(&s);
    println!("{}", word);

}
