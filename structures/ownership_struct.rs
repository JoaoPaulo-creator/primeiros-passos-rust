struct User {
    ativo: bool,
    user_name: &str,
    email: &str,
    logado_na_conta: i64,
}

fn main() {
    let usuario = User {
        email: "email@gmail.com",
        user_name: "username123",
        ativo: true,
        logado_na_conta: 1,
    };
}