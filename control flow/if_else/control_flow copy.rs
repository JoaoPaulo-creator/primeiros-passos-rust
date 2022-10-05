fn main(){
    /*
    let numero: i32 = 3;
    if numero < 5{
        println!("Isso é verdade");
    }else{
        println!("Isso não é verdade");
    }
     */
    //verificar_se_numero_e_verdade();
    if_dentro_da_variavel();
}

#[warn(dead_code)]
fn verificar_se_numero_e_verdade(){
    let numero = 3;
    if numero != 0{
        println!("O número é {numero}");
    }
}


fn if_dentro_da_variavel(){
    let condicao = false;
    let numero = if condicao { 5 } else { 6 };
    print!("{numero}");
}