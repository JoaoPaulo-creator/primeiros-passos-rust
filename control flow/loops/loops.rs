fn main() {
    let result = incrementar_numero(0);
    println!("The result is {result}");
    foo();
}


fn incrementar_numero(x: i64) -> i64{
    let mut contador = x;
    let resultado = loop{
        contador += 1;
        
        if contador == 10{
            break contador * 2;
        }
    };

    return resultado;
}


fn foo(){
    let mut contador = 0;
    'contando: loop{
        println!("Conta = {contador}");
        let mut quanto_falta = 10;

        loop{
            println!("Faltando = {quanto_falta}");
            if quanto_falta == 9{
                break;
            }

            if contador == 2{
                break 'contando;
            }

            quanto_falta -= 1;
        }

        contador += 1;
    }

    println!("Fim da contagem = {contador}");
}