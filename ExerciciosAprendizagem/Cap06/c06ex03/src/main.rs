use std::io;
use std::io::prelude::*;

fn main() {    

    let valor = &10i32;

    match valor {
        &val => println!("Valor por desestruturação (&) ..: {:?}", val),
    }

    match *valor {
        val => println!("Valor por desreferência   (*) ..: {:?}", val),
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
