use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let num: i32;
    let suc: i32;
    let ant: i32;
 
    print!("Entre valor numérico qualquer ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    num = valor.trim().parse::<i32>().unwrap();

    suc = num + 1;
    ant = num - 1;

    println!("Sucessor .......................: {}", suc);
    println!("Antecessor .....................: {}", ant);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
