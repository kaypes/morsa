mod ast;
mod interpreter;
mod lexer;
mod parser;

use chumsky::Parser;
use logos::Logos;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return println!("Uso: cargo run <arquivo.morsa>");
    }

    let code = fs::read_to_string(&args[1]).expect("Erro ao ler arquivo");
    let tokens: Vec<_> = lexer::Token::lexer(&code).filter_map(|r| r.ok()).collect();

    match parser::parser_da_morsa().parse(&tokens).into_result() {
        Ok(ast) => {
            let mut env = interpreter::Environment::new();
            if let Err(e) = env.execute(ast) {
                eprintln!("Erro: {}", e);
            }
        }
        Err(errs) => {
            for e in errs {
                println!("Erro de parsing: {:?}", e);
            }
        }
    }
}
