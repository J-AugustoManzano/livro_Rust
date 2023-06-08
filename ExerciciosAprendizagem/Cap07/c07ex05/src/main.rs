use std::io;
use std::io::prelude::*;
use std::mem;

fn main() {

    let valores: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let tamanho = valores.len();

    for i in 0..tamanho {
        println!("VALORES[{:2}] = {}", i + 1, valores[i]);	
    }

    println!();
    println!(" 1o. elemento = {}", valores[0]);
    println!("10o. elemento = {}", valores[9]);

    println!();
    println!("A matriz possui {} elementos", tamanho);

    println!();
    println!("A matriz ocupa {} bytes", mem::size_of_val(&valores));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
