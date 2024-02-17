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
    let linhas: Vec<String> = entrada();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("\n\tNúmero de argumentos insuficiente.");
        help(Misuse);
        exit(1)
    };
    
    linhas.into_iter().for_each(|mut linha|{
        linha = if args[1].contains("a"){
            capt_primeira(&linha)}else{linha};
        linha = if args[1].contains("e"){
           estranhos(&linha)}else{linha};
        linha = if args[1].contains("s"){
            espacos(&linha)}else{linha};
        linha = if args[1].contains("p"){
            pontos(&linha)}else{linha};
        linha = if args[1].contains("x"){
            separador_camel_case(&linha)}else{linha};
        linha = if args[1].contains("r"){
            repetidos(&linha)}else{linha};
        linha = if args[1].contains("d"){
            remove_diacritics(&linha)}else{linha};
        linha = if args[1].contains("l"){
            minusculas(&linha)}else{linha};
        linha = if args[1].contains("m"){
            comuns(&linha)}else{linha};
        linha = if args[1].contains("m"){
            camel_case(&linha)}else{linha};
        linha = if args[1].contains("u"){
            printaveis(&linha)}else{linha};
        linha = separacao(&linha);
        linha = finaliza(&linha);
        println!("{linha}");
    })
}
