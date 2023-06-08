use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let n: i64;
    let r4: i64;
    let r5: i64;

    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    n = valor.trim().parse::<i64>().unwrap();

    r4 = n - 4 * (n / 4); 
    r5 = n - 5 * (n / 5);

    if r4 == 0 && r5 == 0 {
        println!("{}", n);
    } else {
        println!("Valor não é divisível por 4 e 5.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
