//Uma estrutura comporta "parâmetros" de um "Objeto", onde esses parametros podem serm de tipos
//diferentes


//Minha estrutura

struct User {
    ativo: bool,
    user_name: String,
    email: String,
    logado_na_conta: i64,
}

fn build_user(email: String, user_name: String) -> User {
    User {
        email: email,
        user_name: user_name,
        ativo: true,
        logado_na_conta: 1,
    }
}

fn main() {
    // Atribuindo a struct a uma variável

   /* let mut usuario = User {
        email: String::from("test@gmail.com"),
        user_name: String::from("Joao"),
        ativo: true,
        logado_na_conta: 1,
    };

    usuario.email = String::from("teste@gmail.com");
    println!("{}", usuario.email);
    */

    let user = build_user(
        String::from ("teste@gmail.com"),
        String::from("ze")
    );

    // Atribuindo parte de uma estrutura dentro de outra estrutura
    let user2 = User {
        email: String::from ("teste@teste.com"),
        ..user
    };
}