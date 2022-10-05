fn main(){
    let mut numero = 3;

    while numero != 0{
        println!("{numero}");
        numero -= 1;
    }

    println!("Quitando");

    laco_em_uma_lista();

}


fn laco_em_uma_lista(){
    let lista = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 5{
        println!("O valor é: {}", lista[index]);
        index += 1;
    }
}