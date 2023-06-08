use std::io;
use std::io::prelude::*;

fn pesquisa(matriz: &mut [u32]) {

    let tamanho = matriz.len();
    let mut resposta = String::new(); 
    let mut resp: char; 
    let mut i: usize;
    let mut acha: bool;  
    let mut pesq = String::new(); 
    
    loop {

        i = 0;
        acha = false;
        pesq.clear();
        		
        print!("\nEntre o valor a ser pesquisado: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut pesq).expect("Erro na entrada.");
        let pesq: u32 = match pesq.trim().parse() {
            Ok(numero) => numero,
            Err(_) => 0
        };
        
        while i <= tamanho - 1 && acha == false {
            if pesq == matriz[i] {
                acha = true;
            } else {
                i += 1;
            }
        }

        if acha == true {
            println!("{} foi localizado na {}a. posição.", pesq, i + 1);
        } else {
            println!("{} não foi localizado.", pesq);
        }

        println!();
        println!("Deseja continuar?");
        print!("Tecle [S] para SIM / qualquer caractere para NAO: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut resposta).unwrap();
        resp = resposta.to_uppercase().trim().parse::<char>().unwrap();
        resposta.clear();

        if resp != 'S' { break; }
    }
}

fn main() {
	
    let mut a: [u32;  8] = [0;  8];   
    let mut b: [u32;  8] = [0;  8];   
    
    let mut valor = String::new();
    
    for i in 0 .. a.len() {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<u32>().unwrap();
        valor.clear();
    }

    for i in 0 .. a.len() {
        b[i] = a[i] * 5;
    }
       
    pesquisa(&mut b);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
