use std::io;
use std::io::prelude::*;

fn main() {

    let mut lista = Vec::new();

    lista.push(1); 
    lista.push(2); 
    lista.push(3); 
    lista.push(4); 
    lista.push(5); 

    for i in &lista {
        println!("{}", i);
    }
    println!();

    println!("Tamanho do vetor ...: {:?}", lista.len());
    println!("Primeiro elemento ..: {:?}", lista.first());
    println!("Último elemento ....: {:?}", lista.last());
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
