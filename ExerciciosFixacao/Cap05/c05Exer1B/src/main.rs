use std::io;
use std::io::prelude::*;

fn potencia(b: u64, e: u64) {
    let mut s: u64 = 0;
    let mut p: u64 = 1;
    for i in 1 .. e + 1 {
        p *= b;
    }
    println!("Resultado = {}", p);  
}

fn main() {    

    let mut base = String::new();
    let mut expo = String::new();

    let bas: u64;
    let exp: u64;
 
    print!("Entre o valor da base ......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    bas = base.trim().parse::<u64>().unwrap();
 
    print!("Entre o valor da expoente ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut expo).unwrap();
    exp = expo.trim().parse::<u64>().unwrap();

    potencia(bas, exp);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
