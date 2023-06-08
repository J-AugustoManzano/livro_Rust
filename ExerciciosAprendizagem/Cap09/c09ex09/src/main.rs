use std::io;
use std::io::prelude::*;

fn checapar(x: u32) -> bool {
    return x % 2 == 0;
}

fn main() {

    println!("Somatório dos valores pares entre 0 e 100\n");

    // Forma interativa

    let mut soma: u32 = 0;
    for i in 0 .. 100 {
        if checapar(i) {
            soma += i;
        }
    }
    println!("Soma imperativa ....: {}", soma);

    // Forma funcional

    let soma: u32 = (0 ..)
        .map(|n| n)
        .take_while(|&n| n < 100)
        .filter(|&n| checapar(n))
        .fold(0, |s, i| s + i);
    println!("Soma funcional .....: {}", soma);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
