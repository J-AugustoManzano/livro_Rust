use std::io;
use std::io::prelude::*;

struct Par<CHAVE, VALOR> {
    chv: CHAVE,
    vlr: VALOR
}

impl <CHAVE, VALOR> Par<CHAVE, VALOR> {

    fn pegachave(&self) -> &CHAVE { 
        return &self.chv; 
    }

    fn pegavalor(&self) -> &VALOR { 
        return &self.vlr; 
    }

    fn poechave(&mut self, chv: CHAVE) {
        self.chv = chv;
    }

    fn poevalor(&mut self, vlr: VALOR) {
        self.vlr = vlr;
    }

}

fn main() {

    let mut x1 = Par {chv: 0u8, vlr: "".to_string()};
    let mut x2 = Par {chv: 0u8, vlr: "".to_string()};
    let mut x3 = Par {chv: 0u8, vlr: 0};
    let mut x4 = Par {chv: 0u8, vlr: 0.0};
    
    let mut nome;
    
    nome = String::new();       
    print!("Informe o 1o. nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).expect("Entrada incorreta");

    if let Some('\n') = nome.chars().next_back() {
        nome.pop();
    }
    if let Some('\r') = nome.chars().next_back() {
        nome.pop();
    }

    x1.poechave(1);
    x1.poevalor(nome);    
    
    nome = String::new();       
    print!("Informe o 2o. nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).expect("Entrada incorreta");

    if let Some('\n') = nome.chars().next_back() {
        nome.pop();
    }
    if let Some('\r') = nome.chars().next_back() {
        nome.pop();
    }

    x2.poechave(2);
    x2.poevalor(nome);    

    x3.poechave(3);
    x3.poevalor(10u8);    

    x4.poechave(4);
    x4.poevalor(9.99f64);    

    println!();    
    println!("Chave ..: {}\nNome ...: {}", x1.pegachave(), x1.pegavalor());
    println!();    
    println!("Chave ..: {}\nNome ...: {}", x2.pegachave(), x2.pegavalor());
    println!();    
    println!("Chave ..: {}\nValor ..: {}", x3.pegachave(), x3.pegavalor());
    println!();    
    println!("Chave ..: {}\nValor ..: {}", x4.pegachave(), x4.pegavalor());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
