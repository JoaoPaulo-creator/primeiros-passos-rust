
struct Retangulo {
    altura: u32,
    largura: u32,
}

fn area_com_struct(retangulo: &Retangulo) -> u32 {
    return retangulo.largura * retangulo.altura;
}


fn main(){
    let altura = 20;
    let largura = 100;

    let retangulo_ = (30, 50);

    println!("Area do retangulo em pixels: {}", area_comum(altura, largura));
    println!("Area do retangulo em pixels, utilizando a função com tupla: {}",
             area_com_tuplas(retangulo_));

    let retangulo = Retangulo {
        altura: altura,
        largura: largura,
    };


    println!("Area do retangulo em pixels, utilizando a função com struct: {}",
             area_com_struct(&retangulo));

}

fn area_comum(largura: u32, altura: u32) -> u32 {
    return largura * altura;
}

fn area_com_tuplas(dimensoes: (u32, u32)) -> u32 {
    return dimensoes.0 * dimensoes.1;
}

