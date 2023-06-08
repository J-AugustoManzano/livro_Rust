use std::io;
use std::io::prelude::*;

fn pausa() {
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn troca(a: &mut u64, b: &mut u64) -> bool {
    let x: u64;
    if *a > *b {
        x = *a;
        *a = *b;
        *b = x;
        return true;
    } else {
		return false;
	}
}

fn saida(a: u64, b: u64) {
	println!("N1 = {}", a);
	println!("N2 = {}", b);
}

fn main() {    

    let mut n1 = String::new();
    let mut n2 = String::new();

    println!("Ordena valores\n");
    
    print!("Entre N1: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n1).expect("Erro na entrada.");
    let mut n1: u64 = match n1.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };

    print!("Entre N2: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n2).expect("Erro na entrada.");
    let mut n2: u64 = match n2.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };

    if troca(&mut n1, &mut n2) {
		saida(n1, n2);
		println!("\nHouve troca de valores.");
	} else {
		saida(n1, n2);
		println!("\nNão houve troca de valores.");
    }

    pausa();

}
