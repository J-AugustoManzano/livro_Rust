use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: f64;
    let b: f64;

    let rsom: f64;
    let rsub: f64;
    let rmul: f64;
    let rdiv: f64;

    print!("Entre o valor <A> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<f64>().unwrap();

    print!("Entre o valor <B> .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<f64>().unwrap();

    rsom = a + b;
    rsub = a - b;
    rmul = a * b;
    rdiv = a / b;

    println!();
    println!("Soma ..................: {}", rsom);
    println!("Subtração .............: {}", rsub);
    println!("Multiplicação .........: {}", rmul);
    println!("Divisão ...............: {}", rdiv);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
