use std::io;
use std::io::prelude::*;

fn main() {

    let mut lista = Vec::new();

    lista.push(9); 
    lista.push(8); 
    lista.push(7); 
    lista.push(6); 
    lista.push(5); 

    if lista.is_empty() {
        println!("Vetor sem conteúdo");
    } else {
        println!("Vetor com conteúdo");
    }

    for i in &lista {
        println!("{}", i);
    }
    println!();

    lista.clear();

    if lista.is_empty() {
        println!("Vetor sem conteúdo");
    } else {
        println!("Vetor com conteúdo");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
