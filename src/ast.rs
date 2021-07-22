// #![feature()]

use colored::Colorize;

use crate::{MEMORY, POINTER, cursor::Cursor, debug::Errors, lexer::{Token, Tokens::*}};

#[derive(Debug, Clone)]
pub struct Ast {
    pub inner: Vec<Expr>,
    pub pos: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Dot,
    Comma,
    Plus,
    Minus,
    ShiftR,
    ShiftL,
    Comment(String),
    Loop(Vec<Expr>),
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            inner: Vec::new(),
            pos: (0, 1),
        }
    }

    pub fn build(&mut self, tokens: Vec<Token>, source: &String) -> &mut Self {
        let mut peek = tokens.into_iter().peekable();

        while let Some(token) = peek.next() {
            self.pos.0 += token.span.len() as u32;

            match token.kind {
                Dot => self.inner.push(Expr::Dot),
                Comma => self.inner.push(Expr::Comma),
                Plus => self.inner.push(Expr::Plus),
                ShiftR => self.inner.push(Expr::ShiftR),
                ShiftL => self.inner.push(Expr::ShiftL),

                // Maybe throw error?
                CloseSquare => {
                    Errors::invalid_token(
                        &source[token.span.as_range()],
                        &format!(
                            "Expected token \"{}\" to be present before a closing square bracket.",
                            "[".red().bold()
                        ),
                        self.pos,
                    );

                    std::process::exit(0);
                }

                Minus => self.inner.push(Expr::Minus),
                Comment => self
                    .inner
                    .push(Expr::Comment(source[token.span.as_range()].to_string())),

                Newline => {
                    self.pos.0 -= token.span.len() as u32;
                    self.pos.1 += 1;
                    self.inner
                        .push(Expr::Comment(source[token.span.as_range()].to_string()))
                }

                OpenSquare => {
                    self.inner.push(
                        Cursor::expect(&mut peek, crate::cursor::Outputs::Loop, &mut self.pos)
                            .unwrap(),
                    );
                }
                _ => (),
            }
        }

        self
    }

    pub fn run(&mut self, mut input: Vec<u8>) {
        let mut exprs_iter = self.inner.clone().into_iter();
        unsafe {
            while let Some(expr) = exprs_iter.next() {
                match expr {
                    Expr::Dot => print!("{}", MEMORY[POINTER] as char),
                    Expr::Comma => MEMORY[POINTER] = input.remove(0),
                    Expr::Plus => MEMORY[POINTER] = MEMORY[POINTER].overflowing_add(1).0,
                    Expr::Minus => MEMORY[POINTER] = MEMORY[POINTER].overflowing_sub(1).0,
                    Expr::ShiftR => POINTER = POINTER.overflowing_add(1).0 as u8 as usize,
                    Expr::ShiftL => POINTER = POINTER.overflowing_sub(1).0 as u8 as usize,
                    Expr::Comment(_) => (),
                    Expr::Loop(exprs) => {
                        // println!("{:?}", exprs);
                        while MEMORY[POINTER] != 0 {
                            // println!("{}", POINTER);
                            Self::run(&mut Ast { inner: exprs.clone(), pos: (0, 0) }, input.clone())
                        }
                    },
                }
            }
        }
    }
}
