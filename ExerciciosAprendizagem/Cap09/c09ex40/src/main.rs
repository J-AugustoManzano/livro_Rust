#![allow(unused_assignments)]
use std::io;
use std::io::prelude::*;

fn removespc(mencomspc: &str) -> String {
	
   let mut mensemspc = String::new();

   for caractere in mencomspc.chars() {
      if caractere != ' ' {
         mensemspc.push(caractere);
      }
   }

   return mensemspc;
   
}

fn main() {

    let mut mencomspc = String::new();
    let mut mensemspc = String::new();

    print!("Informe uma mensagem ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mencomspc).expect("Entrada incorreta");

    if let Some('\n') = mencomspc.chars().next_back() {
        mencomspc.pop();
    }
    if let Some('\r') = mencomspc.chars().next_back() {
        mencomspc.pop();
    }

    mensemspc = removespc(&mencomspc);

    print!("Mensagem sem espaços ..: {}", mensemspc.to_uppercase());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
