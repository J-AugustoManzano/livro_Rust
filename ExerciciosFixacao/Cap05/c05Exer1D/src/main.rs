use std::io;
use std::io::prelude::*;

fn positivo(n: i64) {
    if n < 0 {
        print!("{}", n * -1);
    } else {  
        print!("{}", n);  
    }
}

fn main() {    

    let mut valor = String::new();

    let vlr: i64;
 
    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i64>().unwrap();
 
    positivo(vlr);
    println!();

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
