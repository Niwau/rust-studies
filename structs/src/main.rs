/**
 * Structs 
 */

fn main() {
    // Definindo uma Struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Instanciando uma Struct
    let user1 = User {
        username: String::from("user1"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
        active: true,
    };

    // Acessando os campos de uma Struct
    println!("Username: {}", user1.username);
}
