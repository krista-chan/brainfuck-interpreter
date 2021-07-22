use std::iter::Peekable;

use colored::Colorize;

use crate::{ast::Expr, debug::Errors, lexer::{Token, Tokens}};

#[derive(Debug, Clone, Copy)]
pub enum Outputs {
    Loop,
    // Comment,
}

pub struct Cursor;

impl Cursor {
    pub fn expect(
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        expected_output: Outputs,
        pos: &mut (u32, u32),
    ) -> Option<Expr> {
        match expected_output {
            
            Outputs::Loop => {
                // To be subtracted for debugger
                let mut phantom_pos = (0u32, 0u32);

                let mut output = Vec::new();
                let mut safely_closed = false;
                while let Some(token) = tokens.next() {
                    phantom_pos.0 += token.span.len() as u32;
                    match token.kind {
                        Tokens::Dot => output.push(Expr::Dot),
                        Tokens::Plus => output.push(Expr::Plus),
                        Tokens::Minus => output.push(Expr::Minus),
                        Tokens::Comma => output.push(Expr::Comma),
                        Tokens::ShiftR => output.push(Expr::ShiftR),
                        Tokens::ShiftL => output.push(Expr::ShiftL),
                        Tokens::Comment => (),
                        Tokens::Newline => {
                            phantom_pos.0 -= token.span.len() as u32;
                            phantom_pos.1 += 1;
                        }
                        Tokens::OpenSquare => {
                            output.push(Self::expect(tokens, Outputs::Loop, pos).unwrap())
                        }
                        Tokens::CloseSquare => {
                            safely_closed = true;
                            break;
                        }
                        _ => (),
                    }
                }
                if !safely_closed {
                    Errors::invalid_token("[", &format!("Expected token \"{}\" to be present to safely close this loop.", "]".red().bold()), *pos);
                    std::process::exit(0)
                }

                pos.0 += phantom_pos.0;
                pos.1 += phantom_pos.1;

                Some(Expr::Loop(output))
            }
        }
    }
}
