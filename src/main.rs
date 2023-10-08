mod entrada;
use {
    lib::*, 
    lib::Mistake::*, 
    diacritics::*, 
    std::*, 
    std::process::exit,
    entrada::*
};

fn main() {
    let mut linhas: Vec<String> = entrada();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("\n\tNúmero de argumentos insuficiente.");
        help(Misuse);
        exit(1)
    };
    
    linhas = linhas.into_iter()
    .map(|linha| if args[1].contains("e"){
        estranhos(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("s"){
        espacos(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("p"){
        pontos(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("x"){
        separador_camel_case(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("r"){
        repetidos(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("d"){
        remove_diacritics(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("l"){
        minusculas(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("m"){
        camel_case(&linha); comuns(&linha)}else{linha.to_string()})
    .map(|linha| if args[1].contains("u"){
        printaveis(&linha)}else{linha.to_string()})
    .map(|linha| separacao(&linha))
    .map(|linha| finaliza(&linha))
    .collect();
    
    for linha in linhas {
        println!("{}", linha);
    }
}
