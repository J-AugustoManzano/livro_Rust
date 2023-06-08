use std::io;
use std::io::prelude::*;

fn main() {

    let palavra = String::from("COMPUTADOR");

    let parte1 = &palavra[0..3];
    let parte2 = &palavra[3..7];
    let parte3 = &palavra[7..10];

    print!("{} ", parte1);
    print!("{} ", parte2);
    print!("{}", parte3);
    println!();

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
