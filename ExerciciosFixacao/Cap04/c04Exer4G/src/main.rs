use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let mut vlr: f64;
    let mut s: f64;
    let m: f64;

    s = 0.0;
    for i in 1 .. 11 {
        print!("Entre um valor: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        vlr = valor.trim().parse::<f64>().unwrap();
        s += vlr;
        valor.clear();
    }

    m = s / 10.0;
    
    println!("Media ....: {}", m);
    println!("Soma .....: {}", s);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
