use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [String; 4] = Default::default(); 
    let mut b: [String; 3] = Default::default(); 
    let mut c: [String; 7] = Default::default(); 
        
    for i in 0 .. 4 {
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

    for i in 0 .. 3 {
        print!("Entre o {:2}o. nome: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut b[i]).expect("Entrada incorreta");        
        if let Some('\n') = b[i].chars().next_back() {
            b[i].pop();
        }
        if let Some('\r') = a[i].chars().next_back() {
            b[i].pop();
        }
    }
    println!();

    for i in 0 .. 7 {
       if i <= 3 {
           c[i] = a[i].clone();
       } else {
           c[i] = b[i - 4].clone();   
       }
    }

    for i in 0 .. 7 {
        println!("C[{:2}] = {}", i + 1, c[i]);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
