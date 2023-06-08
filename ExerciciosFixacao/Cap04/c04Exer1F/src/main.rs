use std::io;
use std::io::prelude::*;

fn main() {    

    let mut i: u8;
    let mut grao: f64;
    let mut soma: f64;

    grao = 1.0;
    soma = 0.0;
    i = 1;
    while i <= 64 {
        grao *= 2.0;
        soma += grao;
        i += 1;
    }
    println!("{}", soma);
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
