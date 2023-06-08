use std::io;
use std::io::prelude::*;

struct CadAluno {
    nome: String,
    notas: Vec<f32>,
    media: f32,
}

fn main() {

    let mut aluno = CadAluno {
                        nome: "".to_string(), 
                        notas: Vec::new(), 
                        media: 0.
                    };
    let mut nota = String::new();   
    let mut soma: f32 = 0.;
    
    print!("Entre nome do aluno ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut aluno.nome).unwrap();
    
    aluno.notas.resize(4, 0.);
    for i in 0 .. 4 {
        print!("Entre a {}a. nota .....: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nota).unwrap();
        aluno.notas[i] = nota.trim().parse::<f32>().unwrap();
        soma += aluno.notas[i];
        nota.clear();
    }
    aluno.media = soma / 4.0;    
    	
    println!();
    println!("Nome do aluno ........: {}", aluno.nome);
        
    for i in 0 .. 4 {
        println!("{}a. nota .............: {:5.2}", i + 1, aluno.notas[i]);
    }
    println!();
    print!("Média  ...............: {:5.2}", aluno.media);

    if aluno.media >= 6. {
        println!(" => Aprovado.");
    } else {
        println!(" => Reprovado.");
    }		

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
