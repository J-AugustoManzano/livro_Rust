use std::io;
use std::io::prelude::*;

fn positivo(n: i64, r: &mut i64) {
    if n < 0 {
        *r = n * -1;
    } else {  
        *r = n;  
    }
}

fn main() {    

    let mut valor = String::new();
    let mut resp: i64 = 0;
    
    let vlr: i64;
 
    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i64>().unwrap();
 
    positivo(vlr, &mut resp);
    println!("Resultado = {}", resp);  

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
