use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    print!("Informe um valor inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
   
    match valor.trim().parse::<u8>() {
        Ok(vlr) => println!("Valor informado: {}", vlr),
        Err(..) => println!("Não é um valor inteiro.")
    };
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
