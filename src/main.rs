mod ast;
mod cursor;
mod debug;
mod lexer;
use std::{fs::File, io::Read, path::PathBuf};

use clap::{App, Arg};
use lexer::{Lexer, Rule, Tokens};
use regex::Regex;

use crate::{ast::Ast, debug::Errors};

pub static mut MEMORY: [u8; 30000] = [0; 30000];
pub static mut POINTER: usize = 0;

fn main() {
    let matches = App::new("Krista's speedy brainfuck interpreter")
        .about("Fun little project I had been meaning to do for a while")
        .name("bf")
        .args(&[
            Arg::with_name("INPUT_FILE")
                .index(1)
                .help("A path to a brainfuck source file")
                .required_unless("evaluate"),
            Arg::with_name("evaluate")
                .long("evaluate")
                .short("e")
                .takes_value(true)
                .help("Some arbitrary brainfuck to be executed"),
            Arg::with_name("stdin")
                .long("stdin")
                .short("i")
                .takes_value(true)
                .help(r#""stdin" for the comma operator"#),
        ])
        .get_matches();

    // let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_owned();
    let src = if let Some(input) = matches.value_of("INPUT_FILE") {
        let path = PathBuf::from(input);
        if !path.exists() || path.is_dir() {
            Errors::file_not_exists(path);
            std::process::exit(0);
        }

        let mut buf = String::new();
        File::open(path).unwrap().read_to_string(&mut buf).unwrap();

        buf
    } else if let Some(input) = matches.value_of("evaluate") {
        input.to_owned()
    } else {
        "".to_owned()
    };

    let lex = Lexer::build(vec![
        Rule {
            kind: Tokens::Dot,
            re: Regex::new(r"^\.").unwrap(),
        },
        Rule {
            kind: Tokens::Comma,
            re: Regex::new(r"^,").unwrap(),
        },
        Rule {
            kind: Tokens::OpenSquare,
            re: Regex::new(r"^\[").unwrap(),
        },
        Rule {
            kind: Tokens::CloseSquare,
            re: Regex::new(r"^\]").unwrap(),
        },
        Rule {
            kind: Tokens::Plus,
            re: Regex::new(r"^\+").unwrap(),
        },
        Rule {
            kind: Tokens::Minus,
            re: Regex::new(r"^\-").unwrap(),
        },
        Rule {
            kind: Tokens::ShiftR,
            re: Regex::new(r"^>").unwrap(),
        },
        Rule {
            kind: Tokens::ShiftL,
            re: Regex::new(r"^<").unwrap(),
        },
        Rule {
            kind: Tokens::Comment,
            re: Regex::new(r"^[\w ]+").unwrap(),
        },
        Rule {
            kind: Tokens::Newline,
            re: Regex::new(r"^(\n|\r\n)+").unwrap(),
        },
    ])
    .tokenize(&src);
    let mut ast = Ast::new();
    ast.build(lex.clone(), &src.to_owned()).run(matches.value_of("stdin").unwrap_or("").as_bytes().to_vec());
}
