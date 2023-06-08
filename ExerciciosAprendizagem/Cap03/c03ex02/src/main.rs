use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: i32;
    let b: i32;
    let r: i32;

    print!("Entre valor <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    print!("Entre valor <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();

    r = a * b;

    println!();

    if r >= 20 {
        println!("Resultado = {}", r + 5);
    } else {
        println!("Resultado = {}", r - 7);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
