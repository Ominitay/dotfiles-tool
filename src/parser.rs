use std::str::CharIndices;
use std::iter::Peekable;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Lexer
#[derive(Debug)]
enum Token {
    StringLit(String),
    Comment(String),
    Error(usize),
    Newline,
}

struct Lexer {
    pub tokens: Vec<Token>,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            input: input,
        }
    }

    fn lex(&mut self) {
        let mut output = Vec::new();
        let mut chars = self.input.char_indices().peekable();

        loop {
            let nextchar = chars.peek().cloned();
            match nextchar {
                Some((_, '"')) => {
                    chars.next();
                    output.push(Lexer::process_string_lit(&mut chars))
                },
                Some((_, '#')) => {
                    chars.next();
                    output.push(Lexer::process_comment(&mut chars))
                },
                Some((_, '\n')) => {
                    chars.next();
                    output.push(Token::Newline)
                },
                Some((_, ' ')) => {
                    chars.next();
                },
                Some((i, _)) => {
                    chars.next();
                    output.push(Token::Error(i.clone()))
                },
                None => break,
            }
        }

        self.tokens = output;
    }

    fn process_string_lit(chars: &mut Peekable<CharIndices>) -> Token {
        let mut output = String::new();

        loop {
            match chars.next() {
                Some((_, '\"')) => break,
                Some((_, '\\')) => {
                    match chars.next() {
                        Some((_, '\\')) => output.push('\\'),
                        Some((_, 'n')) => output.push('\n'),
                        Some((_, c)) => output.push(c),
                        None => break,
                    }
                }
                Some((_, c)) => output.push(c),
                None => break,
            }
        }

        Token::StringLit(output)
    }

    fn process_comment(chars: &mut Peekable<CharIndices>) -> Token {
        let mut output = String::new();

        loop {
            match chars.next() {
                Some((_, '\n')) => break,
                Some((_, c)) => output.push(c),
                None => break,
            }
        }

        Token::Comment(output)
    }
}

// Parser
// TODO

pub fn main() {
    let path = PathBuf::from("/home/renami/conftest.conf");
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.lex();
    println!("{:?}", lexer.tokens);
}
