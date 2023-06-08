use std::io;
use std::io::prelude::*;
 
fn main() {
	
    let valores1: [i32; 5] = [1, 2, 3, 4, 5];
    let valores2: [i32; 5] = [6, 7, 8, 9, 0];
    let valores3: [i32; 5] = [3, 2, 8, 5, 4];

    let lista = valores1.iter();
    println!("Produto da matriz ...........: {:4}", lista.product::<i32>());	
    println!();

    let lista = valores1.iter();
    println!("Quantidade de elementos .....: {:4}", lista.count());	
    println!();
 
    let lista = valores3.iter();
    println!("Último elementos da lista ...: {:?}", lista.last());	
    println!();
       
    let mut lista = valores3.iter();
    println!("Mostra o 3o. elemento .......: {:?}", lista.nth(2));	
    println!();
       
    let mut lista = valores1.iter().chain(valores2.iter()).skip(5);
    for i in 0 .. 6 {
        println!("Elemento {:2} .................: {:?}", i, lista.next());
    }    	
    println!();

    let mut lista = valores1.iter().zip(valores2.iter());
    for i in 0 .. 5 {
        println!("Elemento {:2} .................: {:?}", i, lista.next());
    }    	
    println!();

    let lista = valores3.iter();
    println!("Maior valor da lista ........: {:?}", lista.max());	
    let lista = valores3.iter();
    println!("Menor valor da lista ........: {:?}", lista.min());	
       
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
