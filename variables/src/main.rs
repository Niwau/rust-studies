fn main() {
    /**
     * 1. Variáveis são imutáveis por padrão
     * 2. Para tornar uma variável mutável, utilize a palavra-chave `mut`
     * 3. Shadowing: você pode declarar uma nova variável com o mesmo nome de uma variável anterior
     * 4. Scope: variáveis são válidas apenas dentro do seu escopo
     */

    // 1. Variáveis são imutáveis por padrão (não é possível reatribuir um valor a uma variável)
    let x = 5;
    println!("O valor de x é: {}", x);

    // 2. Para tornar uma variável mutável, utilize a palavra-chave `mut`
    let mut y = 10;
    y = 15;
    println!("O valor de y é: {}", y);

    // 3. Shadowing: você pode declarar uma nova variável com o mesmo nome de uma variável anterior
    let z = 20;
    let z = z + 5;
    println!("O valor de z é: {}", z);

    // 4. Scope: variáveis são válidas apenas dentro do seu escopo
    let a = 30;
    {
        let b = 35;
        println!("O valor de b é: {}", b);
    }
    
    // println!("O valor de b é: {}", b); // Erro: b não está mais em escopo
}
