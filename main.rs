static EVER:u16 = 128;
fn main() {

    const PI:f32 = 3.14;
    let variavel = 256;
    let verdadeiro:bool = false;
    println!("valor de = {} e tamanho de = {} bytes",variavel, std::mem::size_of_val(&variavel));
    println!("O valor de PI é = {}", PI);
    println!("O valor da variável de nome verdadeiro é = {}", verdadeiro);
    println!("{} : tem o tamanho em memória de {} bytes", EVER, std::mem::size_of_val(&EVER));
}
