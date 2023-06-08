use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();
    let mut valor_d = String::new();

    let a: i64;
    let b: i64;
    let c: i64;
    let d: i64;

    let mut r2: i64;
    let mut r3: i64;
 
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

    print!("Entre o valor <D> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_d).unwrap();
    d = valor_d.trim().parse::<i64>().unwrap();

    println!();
    println!("Resultados");
    println!();
    
    r2 = a % 2;
    r3 = a % 3;
    if r2 == 0 || r3 == 0 {
        println!("{}", a);
    }
    
    r2 = b % 2;
    r3 = b % 3;
    if r2 == 0 || r3 == 0 {
        println!("{}", b);
    }
    
    r2 = c % 2;
    r3 = c % 3;
    if r2 == 0 || r3 == 0 {
        println!("{}", c);
    }
    
    r2 = d % 2;
    r3 = d % 3;
    if r2 == 0 || r3 == 0 {
        println!("{}", d);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
