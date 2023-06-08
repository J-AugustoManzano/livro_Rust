use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let num: u32;
    let res: u32;
 
    print!("Entre um valor numerico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    num = valor.trim().parse::<u32>().unwrap();

    res = num * 2;

    if res > 30 {
        println!("Valor informado {}", res);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
