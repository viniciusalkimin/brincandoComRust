static EVER:u16 = 128;

fn soma(a:i32 , b:i32) -> i32{
 a + b
}


fn main() { 

    const PI:f32 = 3.14;
    let variavel = 256;
    let verdadeiro:bool = false;
    println!("valor de = {} e tamanho de = {} bytes",variavel, std::mem::size_of_val(&variavel));
    println!("O valor de PI é = {}", PI);
    println!("O valor da variável de nome verdadeiro é = {}", verdadeiro);
    println!("{} : tem o tamanho em memória de {} bytes", EVER, std::mem::size_of_val(&EVER));
    println!("Soma tem o valor de = {}", soma(2,2));

    let idade:u16 = 16;
    let esta_autorizado:bool = true;

    if idade >= 18 {
        println!("Venda de bebida liberada.")
    } else if idade >= 16 && esta_autorizado {
        println!("Venda de bebida liberada sob autorizacao dos pais.")
    } else {
        println!("Venda de bebida recusada.")
    }

    let condicao = if PI == 3.14 { true } else { false};

    println!("resultado eh = {}", condicao);

    repeticao();

    println!("Nova fn de repeticao ======> ");

    repeticao_loop();

    println!("Nova fn de repeticao ======> ");

    repeticao_for();

}


fn repeticao() {
    let multiplicador:u16 = 2;
    let mut contador:u16 = 0;

    while contador < 10 {
        contador+=1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);
    }
}

fn repeticao_loop(){
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;  
    loop {          
        contador+=1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);  
        if contador == 10 {
            break;
        }         
    }
}

fn repeticao_for(){
    let multiplicador:u8 = 3;

    for i in 1..11{
        println!("{} x {} = {}", multiplicador, i, multiplicador*i);  
    }
}