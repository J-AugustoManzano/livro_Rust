use std::io;
use std::io::prelude::*;

fn main() {    

    let mut nota1 = String::new();
    let mut nota2 = String::new();
    let mut nota3 = String::new();
    let mut nota4 = String::new();
    let mut notae = String::new();

    let n1: f32;
    let n2: f32;
    let n3: f32;
    let n4: f32;
    let ne: f32;
    let md1: f32;
    let md2: f32;
 
    print!("Entre a nota do 1o. bimestre ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota1).unwrap();
    n1 = nota1.trim().parse::<f32>().unwrap();

    print!("Entre a nota do 2o. bimestre ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota2).unwrap();
    n2 = nota2.trim().parse::<f32>().unwrap();

    print!("Entre a nota do 3o. bimestre ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota3).unwrap();
    n3 = nota3.trim().parse::<f32>().unwrap();

    print!("Entre a nota do 4o. bimestre ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota4).unwrap();
    n4 = nota4.trim().parse::<f32>().unwrap();

    md1 = (n1 + n2 + n3 + n4) / 4.0;

    if md1 >= 7.0 {
        println!();
        println!("Aprovado com média --> {:5.2}", md1);
    } else {
        print!("Entre a nota do exame .........: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut notae).unwrap();
        ne = notae.trim().parse::<f32>().unwrap();
		
        md2 = (ne + md1) / 2.0;
        
        println!();
        if md2 >= 5.0 {		
            println!("Aprovado em exame com média --> {:5.2}", md2);
        } else {
            println!("Reprovado com média --> {:5.2}", md2);
        }		
    }		

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
