use std::io;
use std::io::prelude::*;

fn main() {

    let palavra = String::from("COMPUTADOR");

    let parte1: String = palavra.chars().skip(0).take(3).collect();
    let parte2: String = palavra.chars().skip(3).take(4).collect();
    let parte3: String = palavra.chars().skip(7).take(3).collect();

    print!("{} ", parte1);
    print!("{} ", parte2);
    print!("{}", parte3);
    println!();

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
