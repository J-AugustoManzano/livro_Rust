use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();
    let vlr: u8;
    let r: u8;

    print!("Informe um valor inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
   
    vlr = match valor.trim().parse::<u8>() {
        Ok(vlr) => vlr,
        Err(_)  => 0
    };
    
    r = vlr * 2;

    println!("Resultado = {}", r);
  
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
