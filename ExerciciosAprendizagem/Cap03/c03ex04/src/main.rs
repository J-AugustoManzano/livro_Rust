use std::io;
use std::io::prelude::*;

fn main() {    

    let mut cod = String::new();

    let codigo: i8;

    print!("Entre o código de acesso: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cod).unwrap();
    codigo = cod.trim().parse::<i8>().unwrap();

    println!();

    if codigo == 1 || codigo == 2 || codigo == 3 {
      if codigo == 1 {
        println!("Foi acionado o código: um.");
      }
      if codigo == 2 {
        println!("Foi acionado o código: dois."); 
      }
      if codigo == 3 {
        println!("Foi acionado o código: três.");
      }
    } else {
        println!("Código inválido.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
