use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let numero: i32;

    print!("Entre um valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    numero = valor.trim().parse::<i32>().unwrap();

    println!();

    if numero >= 20 && numero <= 90 {
        println!("O valor está entre 20 e 90.");
    } else {
        println!("O valor não está entre 20 e 90.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
