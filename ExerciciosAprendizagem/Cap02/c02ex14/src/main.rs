/// main.rs 

/* Projeto .........: c02ex14 
 * Autor(es) .......: Augusto Manzano
 * Data ............: 11/9/2017
 * Versao ..........: 1.0
 * Revisao .........: 11/9/2017
 * */
 
use std::*;
use std::io::prelude::*;
 
fn main() {
	
	/// Trecho com apresentação de faixa mínima e máxima de valores
	
    println!("i8 ....: de {:20} até {:20}", i8::MIN, i8::MAX);
    println!("u8 ....: de {:20} até {:20}", u8::MIN, u8::MAX);
    println!();
    println!("i16 ...: de {:20} até {:20}", i16::MIN, i16::MAX);
    println!("u16 ...: de {:20} até {:20}", u16::MIN, u16::MAX);
    println!();
    println!("i32 ...: de {:20} até {:20}", i32::MIN, i32::MAX);
    println!("u32 ...: de {:20} até {:20}", u32::MIN, u32::MAX);
    println!();
    println!("i64 ...: de {:20} até {:20}", i64::MIN, i64::MAX);
    println!("u64 ...: de {:20} até {:20}", u64::MIN, u64::MAX);
    println!();
    println!("isize .: de {:20} até {:20}", isize::MIN, isize::MAX);
    println!("usize .: de {:20} até {:20}", usize::MIN, usize::MAX);

    println!(); 
    print!("Tecle <Enter> para encerrar..."); // Acione apenas <Enter>
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
	
}
