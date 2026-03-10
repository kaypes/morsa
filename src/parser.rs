use chumsky::prelude::*;
use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

type Extra<'a> = extra::Err<Rich<'a, Token>>;

pub fn parser_da_morsa<'a>() -> impl Parser<'a, &'a [Token], Vec<Stmt>, Extra<'a>> {
    
    let ident = select! { 
        Token::Morse(s) if Token::Morse(s.clone()).to_digit().is_none() => s 
    };
    
    let digit = select! { 
        t @ Token::Morse(_) if t.to_digit().is_some() => t.to_digit().unwrap() 
    };

    // Número: Combina dígitos (ex: 9 9 9 -> 999)
    let num = digit
        .repeated()
        .at_least(1)
        .collect::<Vec<i32>>() // Forçamos o tipo da coleção
        .map(|digits: Vec<i32>| digits.iter().fold(0i32, |acc, d| acc * 10 + d));

    let val = num.map(Expr::Int).or(ident.clone().map(Expr::Identifier));

    // Expressões: Soma, Igualdade e Menor Que
    let add = val.clone()
        .then(just(Token::Plus).ignore_then(val).or_not())
        .map(|(a, b)| match b {
            Some(b) => Expr::Add(Box::new(a), Box::new(b)),
            None => a,
        });

    let expr = add.clone()
        .then(
            just(Token::Eq).to(1).or(just(Token::Less).to(2))
            .then(add)
            .or_not()
        )
        .map(|(a, b)| match b {
            Some((1, b)) => Expr::Eq(Box::new(a), Box::new(b)),
            Some((2, b)) => Expr::Less(Box::new(a), Box::new(b)),
            _ => a,
        });

    recursive(|stmt| {
        let decl = just(Token::Var).to(true).or(just(Token::Const).to(false))
            .then(ident.clone())
            .then_ignore(just(Token::IntType))
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|((is_mutable, name), value)| Stmt::Declaration { name, is_mutable, value });

        let assign = ident.clone()
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|(name, value)| Stmt::Assignment { name, value });

        let print = just(Token::Print).ignore_then(expr.clone()).map(Stmt::Print);
        let brk = just(Token::Break).to(Stmt::Break);

        let repeat = just(Token::Repeat)
            .ignore_then(stmt.clone().repeated().collect::<Vec<Stmt>>())
            .then_ignore(just(Token::End))
            .map(|body| Stmt::Repeat { body });

        let if_stmt = just(Token::If)
            .ignore_then(expr.clone())
            .then(stmt.clone().repeated().collect::<Vec<Stmt>>())
            .then(just(Token::Else).ignore_then(stmt.clone().repeated().collect()).or_not())
            .then_ignore(just(Token::End))
            .map(|((condition, then_body), else_body)| Stmt::If { condition, then_body, else_body });

        decl.or(assign).or(print).or(repeat).or(if_stmt).or(brk)
    })
    .repeated()
    .collect::<Vec<Stmt>>()
}