use std::io;
use std::io::prelude::*;

fn somatorio(n: i32, s: &mut  i32) {
    for i in 1 .. n + 1 {
        *s += i;
    }
}

fn main() {    

    let mut valor = String::new();
    let mut resp: i32 = 0;
 
    print!("Entre um valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: i32 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };

    somatorio(valor, &mut resp);
    println!("Resultado = {}", resp);  

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
