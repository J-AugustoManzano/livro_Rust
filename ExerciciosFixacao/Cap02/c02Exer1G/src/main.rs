use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();

    let a: i64;
    let b: i64;
    let c: i64;
    let r: i64;
 
    print!("Entre o valor <A> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i64>().unwrap();

    print!("Entre o valor <B> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i64>().unwrap();

    print!("Entre o valor <C> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<i64>().unwrap();

    r = a.pow(2) + b.pow(2) + c.pow(2);

    println!("Soma dos quadrados ....: {}", r);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
