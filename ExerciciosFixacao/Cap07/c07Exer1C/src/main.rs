use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [f64; 5] = [0.; 5];              
    let mut b: [f64; 5] = [0.; 5];
    let mut c: [f64; 5] = [0.; 5];
    
    let mut valor = String::new();   
    
    for i in 0 .. 5 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f64>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. 5 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        b[i] = valor.trim().parse::<f64>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. 5 {
        c[i] = a[i] - b[i];
    }

    for i in 0 .. 5 {
        println!("C[{:2}] = {}", i + 1, c[i]);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
