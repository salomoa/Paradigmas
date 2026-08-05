use std::io;

fn tabuada(numero: i32){
    for i in 1..11{
        println!("{} x {} = {}", numero, i, numero*i);
    }
}

fn main() {
    let mut input = String::new();

    println!("Você quer a tabuada de que número?");

    io::stdin()
        .read_line(&mut input)
        .unwrap();
        
    let numero: i32 = input
        .trim()
        .parse()
        .unwrap();
    
    tabuada(numero);
}


/* 
Vaga real: https://br.linkedin.com/jobs/view/rust-software-developer-remote-at-yo-it-consulting-4444817211?position=1&pageNum=0&refId=fzccQccuGY5seke2%2BEb5Ng%3D%3D&trackingId=Ost6tJn51R%2BCBMRrq9PoDg%3D%3D
Desenvolvedor de Software Rust, Vaga Remota com remuneração feita através de entregas.
Salário no Brasil: 9k - 16k mês
Salário Global: US$ 60.000 -– US$ 100.000/ano
Paradigma: Rust em sua base é uma linguagem de múltiplos paradigmas, pois é uma linguagem que pode ser utilizada para múltiplas funções ao mesmo tempo. Apesar disso, há muitas pessoas que a dizem como uma linguagem com paradigma Procedural.
*/
