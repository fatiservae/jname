extern crate diacritics;
mod entrada;
use diacritics::*;
use std::*;
use entrada::*;
use regex::Regex;

fn repetidos(input: &str, r: bool) -> String {
 let re = Regex::new(r"[\.-]{1,10}").unwrap_or_else(|e| {
    panic!("A função 'repetidos' recebeu um argumento inválido: {}", e);
    });
 if r == true{
    return re.replace_all(input, "-").to_string()
    }else{return input.to_string()}
}

fn estranhos(input: &str, e: bool) -> String {
  let re = Regex::new(r"[^\w \.-]").unwrap_or_else(|e| {
    panic!("A função 'estranhos' recebeu um argumento inválido: {}", e);
  });
if e == true{
    return re.replace_all(input, "").to_string()
    }else{return input.to_string()}
}

fn espacos(input: &str, s: bool) -> String {
  let re = Regex::new(r"[ _]").unwrap_or_else(|e| {
    panic!("A função 'espacos' recebeu um argumento inválido: {}", e);
  });
if s == true{
    return re.replace_all(input, "-").to_string()
    }else{return input.to_string()}
}

fn separacao(input: &str) -> String { // conserta repeticoes de .- ou -.
    let re = Regex::new(r"[\.-][\.-]").unwrap_or_else(|e| {
    panic!("A função 'separacao' recebeu um argumento inválido: {}", e);
    });
    re.replace_all(input, "-").to_string()
}
fn pontos(input: &str, p: bool) -> String { 
    let re = Regex::new(r"\.").unwrap_or_else(|e| {
    panic!("A função 'pontos' recebeu um argumento inválido: {}", e);
    });
    if p == true{
        return re.replace_all(input, "-").to_string()
    }else{return input.to_string()}
}
fn finaliza(input: &str) -> String { // conserta finais de .- ou -.
    input.trim_end_matches(|c: char| c.is_alphanumeric() == false).to_string()
}
fn minusculas(input: &str) -> String {
    input.to_lowercase().to_string()
}

fn main() {
    let mut linhas: Vec<String> = entrada();
    let (mut d, mut s, mut r, mut e, mut p, mut l, mut nada) = 
        (false, false, false, false, false, false, false); 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {nada = true};
    if nada == false {
       if args[1].contains("d") {d = true};
       if args[1].contains("s") {s = true};
       if args[1].contains("r") {r = true};
       if args[1].contains("p") {p = true};
       if args[1].contains("e") {e = true};
       if args[1].contains("l") {l = true};
    };
    linhas = linhas.into_iter()
                    .map(|x| if d == true {remove_diacritics(&x)}else{x})
                    .map(|x| if l == true {minusculas(&x)}else{x})
                    .map(|x| estranhos(&x, e))
                    .map(|x| espacos(&x, s))
                    .map(|x| pontos(&x, p))
                    .map(|x| separacao(&x))
                    .map(|x| finaliza(&x))
                    .map(|x| repetidos(&x, r))
                    .collect();
    for linha in linhas {
        println!("{}", linha);
    }
}
