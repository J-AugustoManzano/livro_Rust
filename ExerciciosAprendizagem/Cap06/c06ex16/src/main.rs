use std::io;
use std::io::prelude::*;

fn main() {

    // Estilo: Caractere para ASCII

    println!("{}", b'A');
    println!("{}", b'B');
    println!("{}", b'C');

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
