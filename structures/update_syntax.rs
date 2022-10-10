struct User {
    ativo: bool,
    user_name: String,
    email: String,
    logado_na_conta: i64,
}


fn main(){
    let user = User {
        email: String::from("test@gmail.com"),
        user_name: String::from("Joao"),
        ativo: true,
        logado_na_conta: 1,
    };

    let user2 = User{
        ativo: user.ativo,
        user_name: user.user_name,
        email: String::from("test@gmail.com"),
        logado_na_conta: user.logado_na_conta,
    };
}