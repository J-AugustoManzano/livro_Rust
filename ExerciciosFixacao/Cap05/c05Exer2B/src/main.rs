use std::io;
use std::io::prelude::*;

fn potencia(b: u64, e: u64, p: &mut  u64) {
    *p = 1;
    for i in 1 .. e + 1 {
        *p *= b;
    }
}

fn main() {    

    let mut base = String::new();
    let mut expo = String::new();
    let mut resp: u64 = 0;

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

    potencia(bas, exp, &mut resp);
    println!("Resultado = {}", resp);  

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
