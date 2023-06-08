use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let vlr: u8;

    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<u8>().unwrap();

    match vlr {
         1 | 3 | 5 | 7 | 9 => println!("Valor impar."),
         2 | 4 | 6 | 8     => println!("Valor par."),
         _                 => println!("Valor inválido."),
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
