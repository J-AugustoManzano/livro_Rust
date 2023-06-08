use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [u32; 8] = [0; 8];              
    let mut b: [u32; 8] = [0; 8];
    
    let mut valor = String::new();   
    
    for i in 0 .. 8 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<u32>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. 8 {
        b[i] = a[i].pow(2);
    }

    for i in 0 .. 8 {
        println!("A[{:2}] = {:3}", i + 1, a[i]);
    }
    println!();

    for i in 0 .. 8 {
        println!("B[{:2}] = {:3}", i + 1, b[i]);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
