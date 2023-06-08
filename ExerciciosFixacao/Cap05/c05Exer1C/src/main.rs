use std::io;
use std::io::prelude::*;

fn checa(n: u64) {
    if n % 2 == 0 {
        print!("par");
    } else {  
        print!("impar");  
    }
}

fn main() {    

    let mut valor = String::new();

    let vlr: u64;
 
    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<u64>().unwrap();
 
    checa(vlr);
    println!();

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
