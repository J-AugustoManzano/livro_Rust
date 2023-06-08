use std::io;
use std::io::prelude::*;

fn somatorio(n: i32) -> i32 {
    let mut s: i32 = 0;	
    for i in 1 .. n + 1 {
        s += i;
    }
    return s;
}

fn main() {    

    let mut valor = String::new();
    let resp = somatorio;
    
    print!("Entre um valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: i32 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };

    println!("Resultado = {}", resp(valor));  

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
