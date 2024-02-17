use {regex::Regex, std::process::exit};
///Remove pontos, traços e espaços repetidos.
pub fn repetidos(input: &str) -> String {
    let re = Regex::new(r"([\. -]){1,10}").unwrap(); // r"() cria um grupo captura
    re.replace_all(input, "$1").to_string()
}
///Remove caracteres estranhos.
pub fn estranhos(input: &str) -> String {
    let re = Regex::new(r"[^\w \.-]").unwrap();
    re.replace_all(input, "").to_string()
}
///Transforma separações feitas por `_` e espaços em traço.
pub fn espacos(input: &str) -> String {
    let re = Regex::new(r"[ _]").unwrap();
    re.replace_all(input, "-").to_string()
}
///Substitui separações feitas por pontos por traço.
pub fn pontos(input: &str) -> String { 
    let re = Regex::new(r"\.").unwrap();
    re.replace_all(input, "-").to_string()
}
///Elimina separações repetidas de ponto e traços.
///
///TODO: Generalizar o que é uma separação.
pub fn separacao(input: &str) -> String { 
    let re = Regex::new(r"[\.-][\.-]").unwrap();
    re.replace_all(input, "-").to_string()
}
///Separa CamelCases em Camel-Cases.
pub fn separador_camel_case(input: &str) -> String {
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
///Elimina finais contendo pontos ou traços.
pub fn finaliza(input: &str) -> String { // conserta finais de .- ou -.
    input.trim_end_matches(|c: char| 
        c.is_alphanumeric() == false).to_string()
}
///Converte em lowercase (snakecase).
pub fn minusculas(input: &str) -> String {
    input.to_lowercase().to_string()
}
///Remove caracters não printáveis.
pub fn printaveis(input: &str) -> String {
    let mut output: String = String::new();
    for c in input.chars(){
        match c {
            '\t' | '\n' | '\r' => output.push(c),
            _ if (c as u8) >= 0x20 => output.push(c),
            _ if (c as u8) <= 0x7E => output.push(c),
            _ if c.is_ascii() => output.push(c),
            _ => {}
        };
        //output.push(c)
    };
    //for c in input.chars(){
    //    if c.is_ascii() && 
    //    (c == '\t' || c == '\n' || c == '\r' || (c as u8) >= 0x20 && 
    //    (c as u8) <= 0x7E) {
    //        output.push(c)
    //    };
    //}
    output.to_string()
}
///Transforma em CamelCase.
pub fn camel_case(input: &str) -> String {
    let entrada: Vec<char> = input.chars().collect();
    let mut saida: Vec<char> = Vec::new();
    saida.push(entrada[0].to_ascii_uppercase());
    let mut last_char: Option<char> = None;
    for c in &entrada{
        // O last_char recebe um valor se existir e
        // caso esse valor seja um dos mencionados, 
        // o bloco de código com push e uppercase é
        // executado.
        // Caso last_char exista mas não seja um dos
        // mencionados, faz push sem uppercase.
        // Se não existir, o bloco if let nem é executado.
        if let Some(last) = last_char {
            match last {
            ' ' | ',' | '-' | '.' | '_' =>  {
                saida.push(c.to_ascii_uppercase())
            },
            _ => {
                saida.push(c.to_string().chars().next().unwrap());
                }
            };
        }
        last_char = Some(*c); // carrega o char atual como "last"
                              // na próxima. Na primeira vez 
                              // last_char é None
    }
    saida.iter().collect::<String>()
}

pub fn capt_primeira(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars[0] = chars[0].to_ascii_uppercase();
    chars[1..].iter_mut().for_each(|c| *c = c.to_ascii_lowercase());
    chars.into_iter().collect()
}
    
///Opera transformação para lowercase em palavras de até três
///caracteres.
///
///TODO: Algumas palavras de três caracteres não devem ser 
///transformadas. Atualmente o código é greedy e transforma
///todas palavras de até três caracteres.
pub fn comuns(input: &str) -> String {
    let mut output = String::new();
    let mut last_separator = 0;

    for (i, c) in input.char_indices() {
        match c {
            ' ' | '-' | '_' | ',' => {
                let slice = &input[last_separator..i];
                let comum = &slice.to_lowercase();
                output.push_str(match slice.len() {
                    1..=3 => comum,
                    _ => slice,
                });
                output.push(c);
                last_separator = i + 1;
            },
            _ => {}
        }
        //if c == ' ' || c == '-' || c == '_' || c == ',' {
        //    let slice = &input[last_separator..i];
        //    let comum = &slice.to_lowercase();
        //    output.push_str(match slice.len() {
        //        1..=3 => comum,
        //        _ => slice,
        //    });
        //    output.push(c);
        //    last_separator = i + 1;
        //}
    }

    let slice = &input[last_separator..];
    let comum = &slice.to_lowercase();
    output.push_str(match slice.len() {
        1..=3 => comum,
        _ => slice,
    });

    output
}

pub enum Mistake {
    Misuse,
    Help
}

pub fn help(mistake: Mistake) {
    fn msg () {println!(r#"

    Flag    Opção
    -h      Ajuda.
    -l      Transforma para minúsculas.
    -r      Elimina separadores repetidos.
    -e      Elimina caracters estranhos.
    -m      Transforma em CamelCase.
    -x      Separa CamelCase.
    -d      Elimina diacríticos.
    -s      Elimina espaços.
    -p      Elimina pontos.
    -u      Elimina não printáveis.

    "#)}
    match mistake {
        Mistake::Misuse => {msg(); exit(1)},
        Mistake::Help => msg() 
    }
}
