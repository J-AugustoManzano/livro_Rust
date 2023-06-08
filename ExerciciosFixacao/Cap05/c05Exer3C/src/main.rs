use std::io;
use std::io::prelude::*;

fn positivo(n: i64) -> i64 {
    if n < 0 {
        return n * -1;
    } else {  
        return n;  
    }
}

fn main() {    

    let mut valor = String::new();
    
    let vlr: i64;
 
    print!("Entre um valor numérico: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i64>().unwrap();
   
    println!("Resultado = {}",  positivo(vlr));  

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
