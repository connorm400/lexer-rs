#[macro_use]
extern crate lazy_static;
use lex::Lexer;
use std::io;
use std::io::Write;

mod lex;

const PROMPT: &str = ">> ";

fn main() {
    println!("Welcome to monkeylang repl");

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        let mut scanned = String::new();
        io::stdin().read_line(&mut scanned).unwrap();

        let lex = Lexer::new(scanned.trim());

        for token in lex {
            println!("{token},");
        }
        println!("end of file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lex::Token as T;
    #[test]
    fn lex_test() {
        const INPUT: &str =
            r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;"#;
        let correct_tokens = vec![
            T::Let,
            T::Ident("five".into()),
            T::Assign,
            T::Int(5),
            T::Semicolon,
            T::Let,
            T::Ident("ten".into()),
            T::Assign,
            T::Int(10),
            T::Semicolon,
            T::Let,
            T::Ident("add".into()),
            T::Assign,
            T::Function,
            T::LParen,
            T::Ident("x".into()),
            T::Comma,
            T::Ident("y".into()),
            T::RParen,
            T::LBrace,
            T::Ident("x".into()),
            T::Plus,
            T::Ident("y".into()),
            T::Semicolon,
            T::RBrace,
            T::Semicolon,
            T::Let,
            T::Ident("result".into()),
            // we aren't done writing tests
        ];

        let lex = Lexer::new(INPUT);

        for (expected, got) in lex.zip(correct_tokens.iter()) {
            assert_eq!(expected, got.clone());
        }

        todo!("not all tokens are tested");
    }
}