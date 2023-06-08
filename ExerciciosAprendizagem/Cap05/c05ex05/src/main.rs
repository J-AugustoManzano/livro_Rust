use std::io;
use std::io::prelude::*;

fn pausa() {
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn fatorial(n: u64, f: &mut u64) {
    for i in 1 .. n + 1 {
        *f *= i;
    }
}

fn main() {    

    let mut valor = String::new();
    let mut result: u64 = 1;

    print!("Entre o valor da fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: u64 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };

    fatorial(valor , &mut result);

    println!("Resultado = {}", result);  
    pausa();

}
