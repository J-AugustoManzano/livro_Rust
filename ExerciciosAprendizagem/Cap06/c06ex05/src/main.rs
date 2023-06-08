use std::io;
use std::io::prelude::*;

fn main() {    

    let valor = 10i32;

    match valor {
        ref val => println!("Valor obtido por referência: {:?}", val),
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
