use std::io;
use std::io::prelude::*;

struct Idade(u16);

fn main() {
	
    let mut ano_nasci = String::new();
    let mut ano_atual = String::new();
    
    let an: u16;
    let aa: u16;

    print!("Entre ano de nascimento ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ano_nasci).unwrap();
    an = ano_nasci.trim().parse::<u16>().unwrap();

    print!("Entre ano atual ..........: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ano_atual).unwrap();
    aa = ano_atual.trim().parse::<u16>().unwrap();

    let idade = Idade(aa - an);

    let Idade(tempo_de_vida) = idade;
    
    println!();
    println!("Sua idade é {} anos.", tempo_de_vida);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
