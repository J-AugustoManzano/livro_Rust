use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();
    let mut valor_d = String::new();
    let mut valor_e = String::new();

    let a: u32;
    let b: u32;
    let c: u32;
    let d: u32;
    let e: u32;

    let mut x: u32;
    let mut y: u32;
 
    print!("Entre o valor <A> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<u32>().unwrap();

    print!("Entre o valor <B> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<u32>().unwrap();

    print!("Entre o valor <C> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<u32>().unwrap();

    print!("Entre o valor <D> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_d).unwrap();
    d = valor_d.trim().parse::<u32>().unwrap();

    print!("Entre o valor <E> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_e).unwrap();
    e = valor_e.trim().parse::<u32>().unwrap();

    x = a;
    
    if x < b {
        x = b;
    }

    if x < c {
        x = c;
    }

    if x < d {
        x = d;
    }

    if x < e {
        x = e;
    }

    y = a;
    
    if y > b {
        y = b;
    }

    if y > c {
        y = c;
    }

    if y > d {
        y = d;
    }

    if y > e {
        y = e;
    }

    println!();
    println!("Maior valor = {}", x);
    println!("Menor valor = {}", y);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
