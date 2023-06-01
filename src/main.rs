extern crate diacritics;
mod entrada;
use diacritics::*;
use std::*;
use entrada::*;
use regex::Regex;
fn repetidos(input: &str) -> String {
    let re = Regex::new(r"([\. -]){1,10}").unwrap(); // r"() cria um grupo captura
    re.replace_all(input, "$1").to_string()
}
fn estranhos(input: &str) -> String {
    let re = Regex::new(r"[^\w \.-]").unwrap();
    re.replace_all(input, "").to_string()
}
fn espacos(input: &str) -> String {
    let re = Regex::new(r"[ _]").unwrap();
    re.replace_all(input, "-").to_string()
}
fn pontos(input: &str) -> String { 
    let re = Regex::new(r"\.").unwrap();
    re.replace_all(input, "-").to_string()
}
fn separacao(input: &str) -> String { // conserta repeticoes de .- ou -.
    let re = Regex::new(r"[\.-][\.-]").unwrap();
    re.replace_all(input, "-").to_string()
}
fn separador_camel_case(input: &str) -> String {
    let mut result = String::new();
    let input_vec: Vec<char> = input.chars().collect();
    result.push(input_vec[0]);
    for character in &input_vec[1..] {
        if character.is_uppercase() {
            result.push('-');
            result.push(*character);
        }else{ result.push(*character)} 
    }
    result
}
fn finaliza(input: &str) -> String { // conserta finais de .- ou -.
    input.trim_end_matches(|c: char| c.is_alphanumeric() == false).to_string()
}
fn minusculas(input: &str) -> String {
    input.to_lowercase().to_string()
}
fn printaveis(input: &str) -> String {
    let mut output: String = String::new();
    for c in input.chars(){
        if c.is_ascii() && (c == '\t' || c == '\n' || c == '\r' || (c as u8) >= 0x20 && (c as u8) <= 0x7E) {output.push(c)};
    }
    return output.to_string()
}
fn camel_case(input: &str) -> String {
    let entrada: Vec<char> = input.chars().collect();
    let mut saida: Vec<char> = Vec::new();
    saida.push(entrada[0].to_uppercase().to_string().chars().next().unwrap());
    let mut last_char: Option<char> = None;
    for c in &entrada{
        //  * o last_char recebe um valor se existir e
        // caso esse valor seja um dos mencionados, 
        // o bloco de código com push e uppercase é
        // executado.
        //  * caso last_char exista mas não seja um dos
        // mencionados, faz push sem uppercase.
        //  * se não existir, o bloco if let nem é executado.
        if let Some(last) = last_char {
            if last == ' ' || last == ',' || last == '-' || last == '.' || last == '_' {
                saida.push(c.to_uppercase().to_string().chars().next().unwrap());
            }else{
                saida.push(c.to_string().chars().next().unwrap());
            }
        }
        last_char = Some(*c); // carrega o char atual como "last"
                              // na próxima. Na primeira vez 
                              // last_char é None
        }

    saida.iter().collect::<String>()
}

fn comuns(input: &str) -> String {
    let mut output = String::new();
    let mut last_separator = 0;

    for (i, c) in input.char_indices() {
        if c == ' ' || c == '-' || c == '_' || c == ',' {
            let slice = &input[last_separator..i];
            let comum = &slice.to_lowercase();
            output.push_str(match slice.len() {
                1..=3 => comum,
                _ => slice,
            });
            output.push(c);
            last_separator = i + 1;
        }
    }

    let slice = &input[last_separator..];
    let comum = &slice.to_lowercase();
    output.push_str(match slice.len() {
        1..=3 => comum,
        _ => slice,
    });

    output
}


fn main() {
    let mut linhas: Vec<String> = entrada();
    let (mut m, mut u, mut d, mut s, mut r, mut e, mut p, mut l, mut x, mut nada) = 
        (false, false, false, false, false, false, false, false, false, false); 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {nada = true};
    if nada == false {
       if args[1].contains("d") {d = true};
       if args[1].contains("s") {s = true};
       if args[1].contains("r") {r = true};
       if args[1].contains("p") {p = true};
       if args[1].contains("e") {e = true};
       if args[1].contains("l") {l = true};
       if args[1].contains("x") {x = true};
       if args[1].contains("u") {u = true};
       if args[1].contains("m") {m = true};
    };
    linhas = linhas.into_iter()
                    .map(|linha| if e == true {estranhos(&linha)}else{linha})
                    .map(|linha| if s == true {espacos(&linha)}else{linha})
                    .map(|linha| if p == true {pontos(&linha)}else{linha})
                    .map(|linha| if x == true {separador_camel_case(&linha)}else{linha})
                    .map(|linha| if r == true {repetidos(&linha)}else{linha})
                    .map(|linha| if d == true {remove_diacritics(&linha)}else{linha})
                    .map(|linha| if l == true {minusculas(&linha)}else{linha})
                    .map(|linha| if m == true {camel_case(&linha)}else{linha})
                    .map(|linha| if m == true {comuns(&linha)}else{linha})
                    .map(|linha| if u == true {printaveis(&linha)}else{linha})
                    .map(|linha| separacao(&linha))
                    .map(|linha| finaliza(&linha))
                    .collect();
    for linha in linhas {
        println!("{}", linha);
    }
}
