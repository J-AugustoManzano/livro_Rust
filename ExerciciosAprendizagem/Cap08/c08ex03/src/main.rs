use std::io;
use std::io::prelude::*;

fn main() {
	
    let coordenada = (-1, 1);

    match coordenada {
        ( 1, 1) => println!("Primeiro quadrante"),
        (-1, 1) => println!("Segundo quadrante"),
        (-1,-1) => println!("Terceiro quadrante"),
        ( 1,-1) => println!("Quarto quadrante"),
        _       => {},
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
