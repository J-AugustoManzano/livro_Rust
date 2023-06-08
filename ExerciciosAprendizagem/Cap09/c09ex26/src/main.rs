use std::io;
use std::io::prelude::*;

fn main() {

    let cadeia = "Alô, Mundo!";

    println!("Número de caracteres ..: {}", cadeia.chars().count());
    println!("Número de bytes .......: {}", cadeia.len());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
