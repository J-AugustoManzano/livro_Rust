use std::io;
use std::io::prelude::*;

fn main() {    

    let mut numero = String::new();

    let num: u64;
    let res: u64;
 
    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numero).unwrap();
    num = numero.trim().parse::<u64>().unwrap();

    res = num.pow(2);

    println!("O quadrado de {} é igual a {}.", num, res);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
