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

    // Implementation Blocks
    impl User {
        fn new(username: String, email: String) -> User {
            User {
                username,
                email,
                sign_in_count: 1,
                active: true,
            }
        }

        fn get_username(&self) -> &String {
            &self.username
        }

        fn get_email(&self) -> &String {
            &self.email
        }

        fn get_sign_in_count(&self) -> u64 {
            self.sign_in_count
        }

        fn is_active(&self) -> bool {
            self.active
        }

        fn set_username(&mut self, username: String) {
            self.username = username;
        }

        fn set_email(&mut self, email: String) {
            self.email = email;
        }

        fn set_sign_in_count(&mut self, sign_in_count: u64) {
            self.sign_in_count = sign_in_count;
        }

        fn set_active(&mut self, active: bool) {
            self.active = active;
        }
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
