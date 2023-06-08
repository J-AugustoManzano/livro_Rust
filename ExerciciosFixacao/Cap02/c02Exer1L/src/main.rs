use std::io;
use std::io::prelude::*;

fn main() {    

    let mut base = String::new();
    let mut expo = String::new();

    let b: u32;
    let e: u32;
    let r: u32;
 
    print!("Entre o valor da base ......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    b = base.trim().parse::<u32>().unwrap();

    print!("Entre o valor do expoente ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut expo).unwrap();
    e = expo.trim().parse::<u32>().unwrap();

    r = b.pow(e);

    println!("Potência ...................: {}", r);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
