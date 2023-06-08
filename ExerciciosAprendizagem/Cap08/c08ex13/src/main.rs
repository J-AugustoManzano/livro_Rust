use std::io;
use std::io::prelude::*;

struct ClsPessoa {
    nome  : String,
    idade : i8,
}

impl ClsPessoa {

    pub fn new() -> ClsPessoa {
        ClsPessoa {
            nome  : "".to_string(), 
            idade : 0, 
        }
    }

    pub fn peganome(&self) -> String {
        return format!("{}", self.nome)
    }

    pub fn pegaidade(&self) -> i8 {
        return self.idade
    }

    pub fn poenome(&mut self, nome: String) {
        self.nome = nome; 	
    }

    pub fn poeidade(&mut self, idade: i8) {
        self.idade = idade;
    }

}

fn main() {

    let mut pessoa = ClsPessoa::new();

    let mut entra_nm = String::new();
    let mut entra_id = String::new();
    let id: i8;

    print!("Entre o nome ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entra_nm).expect("Entrada incorreta");

    if let Some('\n') = entra_nm.chars().next_back() {
        entra_nm.pop();
    }
    if let Some('\r') = entra_nm.chars().next_back() {
        entra_nm.pop();
    }
    
    print!("Entre a idade ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entra_id).unwrap();
    id = entra_id.trim().parse::<i8>().unwrap();
    
    pessoa.poenome(entra_nm);
    pessoa.poeidade(id);

    println!();
    println!("Nome ...........: {}", pessoa.peganome());
    println!("Idade ..........: {}", pessoa.pegaidade());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
