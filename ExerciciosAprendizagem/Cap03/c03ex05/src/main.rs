use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let vlr: i64;

    print!("Entre valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i64>().unwrap();

    println!();

    if !(vlr >= 10) {
        println!("Valor informado = {}.", vlr);
    } else {
        println!("Valor inválido.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
