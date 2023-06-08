use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [f64; 8] = [0.; 8];              
    let mut b: [f64; 8] = [0.; 8];
    
    let mut valor = String::new();   
    
    for i in 0 .. 8 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f64>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. 8 {
        b[i] = a[7 - i];
    }

    for i in 0 .. 8 {
        println!("A[{}] = {:5.2} | B[{}] = {:5.2}", i + 1, a[i], i + 1, b[i]);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
