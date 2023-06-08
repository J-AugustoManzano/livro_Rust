use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let num: i64;
 
    print!("Entre um valor numerico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    num = valor.trim().parse::<i64>().unwrap();

    if !(num > 3) {
        println!("Valor {}", num);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
