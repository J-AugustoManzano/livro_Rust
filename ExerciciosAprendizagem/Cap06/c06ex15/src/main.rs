use std::io;
use std::io::prelude::*;
use std::char::from_u32;

fn main() {

    // Estilo: Caractere para ASCII

    println!("{}", 'A' as u8);
    println!("{}", 'B' as u8);
    println!("{}", 'C' as u8);

    // Estilo: ASCII para Caractere

    println!("{}", 65 as char);
    println!("{}", 66 as char);
    println!("{}", 67 as char);
 
    // Estilo: Caractere para Unicode

    println!("{}", 'Á' as u32); 
    println!("{}", 'É' as u32); 
    println!("{}", '©' as u32); 

    // Estilo: Unicode para Caractere

    println!("{}", from_u32(193).unwrap());
    println!("{}", from_u32(201).unwrap());
    println!("{}", from_u32(169).unwrap());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
