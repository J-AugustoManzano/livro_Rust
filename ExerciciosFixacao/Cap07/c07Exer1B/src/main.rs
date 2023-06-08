use std::io;
use std::io::prelude::*;

fn main() {

    let mut a = [0; 10];                
    let mut b = [0; 10];
    
    let mut valor = String::new();   
    
    for i in 0 .. 10 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. 10 {
        b[i] = a[i] * 3;
    }

    for i in 0 .. 10 {
        println!("B[{:2}] = {:4} na posição {}.", i + 1, b[i], i);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
