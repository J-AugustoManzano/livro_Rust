use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [String; 10] = Default::default(); 
        
    for i in 0 .. 10 {
        print!("Entre o {:2}o. nome: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut a[i]).expect("Entrada incorreta");        
        if let Some('\n') = a[i].chars().next_back() {
            a[i].pop();
        }
        if let Some('\r') = a[i].chars().next_back() {
            a[i].pop();
        }
    }
    println!();

    for i in 0 .. 10 {
        println!("A[{:2}] = {}", i + 1, a[i]);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
