use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let antecessor = |x: i64| -> i64 {x - 1};

    print!("Entre um valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: i64 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
    
    println!("Antecessor de {} igual a {:?}.", valor, antecessor(valor));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
