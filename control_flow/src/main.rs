/**
 * 1. if/else
 * 2. loops
 * 3. while
 * 4. for
 */

fn main() {

    // if/else
    let number = 10;

    if number < 10 {
        println!("O número é menor que 10");
    } else if number > 10 {
        println!("O número é maior que 10");
    } else {
        println!("O número é igual a 10");
    }

    // também podemos usar if/else em uma expressão
    // note que o tipo de retorno deve ser o mesmo e não pode haver ponto e vírgula
    let result = if number < 10 {
        "menor que 10"
    } else if number > 10 {
        "maior que 10"
    } else {
        "igual a 10"
    };

    println!("O número é {}", result);

    // loops
    let mut counter = 0;

    // loop infinito
    loop {
        println!("O contador é: {}", counter);
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    // dá para usar o loop para retornar um valor
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // para quebrar o loop de fora de um loop aninhado, podemos usar o label 'outer
    'outer: loop {
        println!("Loop externo");

        loop {
            println!("Loop interno");
            break 'outer; // Esse break quebra o loop externo
        }
    }

    // while
    let mut counter = 0;

    while counter < 10 {
        println!("O contador é: {}", counter);
        counter += 1;
    }
    

    // for
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers {
        println!("O número é: {}", number);
    }

}
