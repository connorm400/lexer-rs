use lex::Lexer;

mod lex;

const INPUT: &str =
    r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);"#;


fn main() {
    println!("Lexing: \"{INPUT}\":");

    for token in Lexer::new(INPUT) {
        println!("{token:?},");
    }

    println!("end of file");
}

