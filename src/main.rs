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

    let linguagem = "Lua";

    let proposito = match linguagem {
        "Java" => "Multi proposito",
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Swift" => "IOS",
        _ => "Não conhecida"
    };

    println!("A linguagem {} tem o proposito {}.", linguagem, proposito);

    let mut qualquer_string = String::from("Vinicius");

   // rouba_uma_string(qualquer_string.clone());

    rouba_uma_string( &qualquer_string);

    println!("{}",qualquer_string);

    rouba_uma_string2(&mut qualquer_string);

    println!("{}", qualquer_string); 

    pattern_match();

    erros();
}

fn rouba_uma_string(string: &String) {
    println!("{}", string);
}

fn rouba_uma_string2(string: &mut String) {
    string.push_str(" Alkimin");
    println!("{}", string);
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

fn pattern_match() {
    for x in 1..11 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Mais ou menos",
            4..=5 => "Um monte",
            _ if x % 2 == 0 => "Par é pakas",
            _ => "Bastante"
        });

    }
}

fn erros() {
   // panic!("Testando erro proposital!");
   match resultado() {
    Ok(s) => println!("String de sucesso: {}", s),
    Err(num) => println!("String de erro: Cod{}", num)
   }
}

fn resultado() -> Result<String, u16>{
    //Ok(String::from("Tudo certo!"))
    Err(404)
}