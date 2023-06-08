use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();
    let vlr: i8;

    print!("Informe um valor inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i8>().unwrap();
   
    println!("Valor informado: {}", vlr);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
