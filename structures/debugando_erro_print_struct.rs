
#[derive(Debug)]
struct Retangulo {
    area: u32,
    altura: u32,
}


fn main() {
    let retangulo = Retangulo {
        area: 20,
        altura: 50
    };

    println!("O retangulo é: {:?}", retangulo);
}
