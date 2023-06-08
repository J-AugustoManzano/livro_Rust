use std::io;
use std::io::prelude::*;

fn main() {

    let a = 3;
    let mut pb = Box::new(a);

    println!("Variável [a] = {}", a);
    println!("Ponteiro [b] = {}", pb);

    *pb = 45;
    
    println!();
    println!("Variável [a] = {}", a);
    println!("Ponteiro [b] = {}", pb);
    
    pb = Box::new(9);

    println!();
    println!("Variável [a] = {}", a);
    println!("Ponteiro [b] = {}", pb);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
