use std::io;
use std::io::prelude::*;

fn main() {

    let mut lista = Vec::new();

    lista.push(9); 
    lista.push(7); 
    lista.push(8); 
    lista.push(5); 

    println!("Listagem sem ordenação");
    println!();

    for i in &lista {
        println!("{}", i);
    }
    println!();

    lista.sort();

    println!("Listagem com ordenação crescente");
    println!();

    for i in &lista {
        println!("{}", i);
    }
    println!();

    lista.reverse();

    println!("Listagem com ordenação decrescente");
    println!();

    for i in &lista {
        println!("{}", i);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
