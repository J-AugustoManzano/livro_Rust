use std::io;
use std::io::prelude::*;

fn pesqbin(matriz: &mut [String]) {

    let tamanho = matriz.len();
    let mut resposta = String::new(); 
    let mut resp: char; 
    let mut inicio: usize;
    let mut fim: usize;
    let mut meio: usize;
    let mut acha: bool;  
    let mut pesq = String::new(); 
    let mut aux;

    for i in 0 .. tamanho {
        aux = i;
        for j in i + 1 .. tamanho {
            if matriz[aux] > matriz[j] { 
                aux = j;
            }
        }
        matriz.swap(i, aux);
    }
    
    loop {

        inicio = 0;
        meio = 0;
        fim = tamanho - 1;
        acha = false;
        pesq.clear();
        		
        print!("\nEntre o nome a ser pesquisado: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut pesq).expect("Erro na entrada.");
        if let Some('\n') = pesq.chars().next_back() {
            pesq.pop();
        }
        if let Some('\r') = pesq.chars().next_back() {
            pesq.pop();
        }

        while inicio <= fim && acha == false {
            meio = (inicio + fim) / 2;
            if pesq == matriz[meio] {
                acha = true;
            } else {
                if pesq < matriz[meio] {
                    fim = meio - 1;
                } else {
                    inicio = meio + 1;
                }
            }
        }

        if acha == true {
            println!("{} foi localizado na {}a. posição.", pesq, meio + 1);
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
	
    let mut nomes: [String; 10] = Default::default();                 

    for i in 0 .. 10 {
        print!("Entre o {:2}o. nome: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nomes[i]).expect("Entrada incorreta");        
        if let Some('\n') = nomes[i].chars().next_back() {
            nomes[i].pop();
        }
        if let Some('\r') = nomes[i].chars().next_back() {
            nomes[i].pop();
        }
    }

    pesqbin(&mut nomes);
   
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
