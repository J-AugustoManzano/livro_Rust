use std::io;
use std::io::prelude::*;

fn main() {    

    let mut valor = String::new();

    let fatorial = |n| {
        let mut f: u64 = 1;
        let limite: u64 = n;
        for i in 1 .. limite + 1 {
            f *= i;
        };
        return f; // ou f; (sem o 'return')
    };

    print!("Entre um valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: u64 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
    
    println!("Fatorial de {} igual a {:?}.", valor, fatorial(valor));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
