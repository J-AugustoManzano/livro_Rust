use std::io;
use std::io::prelude::*;

fn main() {

    let mut lista = Vec::new();

    lista.push(9); 
    lista.push(8); 
    lista.push(7); 
    lista.push(6); 
    lista.push(5); 
    lista.push(4); 
    lista.push(3); 
    lista.push(2); 
    lista.push(1); 
    lista.push(0); 
    lista.push(9); 
    lista.push(8); 

    for i in &lista {
        println!("{}", i);
    }
    println!();
    
    lista.pop(); 
    lista.pop(); 
    lista.pop(); 

    for i in &lista {
        println!("{}", i);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
