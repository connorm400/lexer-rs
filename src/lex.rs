use std::fmt::Formatter;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i32),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    LT,
    GT,
    EQ,
    NotEQ,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

// I need to use lazy_static to get a
lazy_static! {
    static ref IDENTIFIERS: Mutex<HashMap<&'static str, Token>> = {
        let mut m = HashMap::new();
        m.insert("fn", Token::Function);
        m.insert("let", Token::Let);
        m.insert("true", Token::True);
        m.insert("false", Token::False);
        m.insert("if", Token::If);
        m.insert("else", Token::Else);
        m.insert("return", Token::Return);
        Mutex::new(m)
    };
}

impl Token {
    fn is_identifier_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == '?' || c == '!'
    }

    fn new_ident(string: &str) -> Token {
        let map = IDENTIFIERS.lock().unwrap();

        map.get(string).unwrap_or(&Token::Ident(string.into())).clone()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        }
    }

    fn read_char(&mut self) {
        self.ch = self.input
            .chars()
            .nth(self.read_position)
            .unwrap_or('\0');

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' if self.peak_char() == '=' => Token::EQ,
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' if self.peak_char() == '=' => Token::NotEQ,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT,
            '>' => Token::GT,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::Eof,
            // for these next two we need to do an early return since the next char has already been read
            _ if Token::is_identifier_letter(self.ch) => {
                return self.read_identifier();
            }
            _ if self.ch.is_digit(10) => {
                return self.read_integer();
            }
            _ => Token::Illegal
        };

        self.read_char();
        return tok;
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while Token::is_identifier_letter(self.ch) {
            self.read_char();
        }

        return Token::new_ident(&self.input[position..self.position]);
    }

    fn read_integer(&mut self) -> Token {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        return Token::Int(self.input[position..self.position]
            .parse()
            .expect("number too big or something"));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::Eof => None,
            x => Some(x),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "Identifier: {s}"),
            Token::Int(i) => write!(f, "Integer: {i}"),
            tok => write!(f, "{tok:?}"),
        }
    }
}