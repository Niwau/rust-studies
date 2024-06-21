/**
 * 1. Booleanos
 * 2. Unsigned integers (Números inteiros positivos)
 * 3. Signed integers (Números inteiros positivos e negativos)
 * 4. Floating-point numbers (Números de ponto flutuante)
 * 5. Platform-specific types
 * 6. Strings e caracteres
 * 7. Arrays
 * 8. Tuples
 * 9. Type Alias
 */

fn main() {

    // 1. Booleanos
    let is_true = true;
    let is_false: bool = false;

    // 2. Unsigned integers
    let x: u8 = 255; // 0 - 255
    let y: u16 = 65535; // 0 - 65535

    // 3. Signed integers
    let a: i8 = -128; // -128 - 127
    let b: i16 = -32768; // -32768 - 32767

    // 4. Floating-point numbers
    let c: f32 = 3.14; // 32-bit floating-point number

    // 5. Platform-specific types
    let d: isize = 10; // 32-bit or 64-bit depending on the platform

    // 6. Strings e caracteres
    let e: char = 'a';
    let f: &str = "Hello, world!";
    let g: String = String::from("Hello, world!");

    // 7. Arrays
    let h: [i32; 5] = [1, 2, 3, 4, 5];
        // ou
    let i = [1, 2, 3, 4, 5];

    // 8. Tuples
    let j: (i32, f64, u8) = (500, 6.4, 1);
        // Desestruturação de tuplas
    let (k, l, m) = j;
        // Unit (É semelhante a um void em outras linguagens de programação)
    let n: () = ();

    // 9. Type Alias
    type Age = u32;
    let o: Age = 30;
    

}
