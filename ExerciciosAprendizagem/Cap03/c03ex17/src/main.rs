use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();
    let r: u8;

    print!("Informe um valor inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
   
    if let Ok(vlr) = valor.trim().parse::<u8>() {
        r = vlr * 2;
        println!("Resultado = {}", r);
    } else {
        println!("Não entrou valor inteiro.")
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
