use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();

    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut x: u32;
 
    print!("Entre o valor numerico <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<u32>().unwrap();

    print!("Entre o valor numerico <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<u32>().unwrap();

    print!("Entre o valor numerico <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<u32>().unwrap();

    if a > b {
        x = a;
        a = b;
        b = x;
    }
    
    if a > c {
        x = a;
        a = c;
        c = x;
    }
    
    if b > c {
        x = b;
        b = c;
        c = x;
    }
    
    println!();
    println!("Os valore ordenados sao:");
    println!();
    println!("{}\n{}\n{}", a, b, c);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
