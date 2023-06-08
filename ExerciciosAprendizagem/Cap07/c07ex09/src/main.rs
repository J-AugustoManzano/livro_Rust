use std::io;
use std::io::prelude::*;

fn main() {

    let valores = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
    let parte = &valores [1..4]; 
    let total = &valores [..];  

    println!("Parte .........: {:?}", parte);
    println!("Total .........: {:?}", total);
    println!("2o. elemento ..: {:?}", &valores [1..2]);
    println!("3o. elemento ..: {:?}", &valores [2..3]);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
