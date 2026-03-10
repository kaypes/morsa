use crate::ast::{Expr, Stmt};
use crate::lexer::Token;
use chumsky::prelude::*;

type Extra<'a> = extra::Err<Rich<'a, Token>>;

pub fn parser_da_morsa<'a>() -> impl Parser<'a, &'a [Token], Vec<Stmt>, Extra<'a>> {
    let ident = select! {
        Token::Morse(s) if Token::Morse(s.clone()).to_digit().is_none() => s
    };

    let digit = select! {
        t @ Token::Morse(_) if t.to_digit().is_some() => t.to_digit().unwrap()
    };

    let num = digit
        .repeated()
        .at_least(1)
        .collect::<Vec<i32>>()
        .map(|digits: Vec<i32>| digits.iter().fold(0i32, |acc, d| acc * 10 + d));

    let expr = recursive(|expr| {
        let val = num.map(Expr::Int).or(ident
            .clone()
            .then(
                just(Token::LBracket)
                    .ignore_then(expr.clone())
                    .then_ignore(just(Token::RBracket))
                    .or_not(),
            )
            .map(|(name, idx)| match idx {
                Some(i) => Expr::ArrayAccess(name, Box::new(i)),
                None => Expr::Identifier(name),
            }));

        let term = val
            .clone()
            .then(
                choice((just(Token::Star).to(1), just(Token::Slash).to(2)))
                    .then(val.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(a, b)| {
                b.into_iter().fold(a, |acc, (op, next)| match op {
                    1 => Expr::Mul(Box::new(acc), Box::new(next)),
                    _ => Expr::Div(Box::new(acc), Box::new(next)),
                })
            });

        let math = term
            .clone()
            .then(
                choice((just(Token::Plus).to(1), just(Token::Minus).to(2)))
                    .then(term.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(a, b)| {
                b.into_iter().fold(a, |acc, (op, next)| match op {
                    1 => Expr::Add(Box::new(acc), Box::new(next)),
                    _ => Expr::Sub(Box::new(acc), Box::new(next)),
                })
            });

        math.clone()
            .then(
                choice((just(Token::Eq).to(1), just(Token::Less).to(2)))
                    .then(math)
                    .or_not(),
            )
            .map(|(a, b)| match b {
                Some((1, b)) => Expr::Eq(Box::new(a), Box::new(b)),
                Some((2, b)) => Expr::Less(Box::new(a), Box::new(b)),
                _ => a,
            })
    });

    recursive(|stmt| {
        let arr_decl = just(Token::IntType)
            .ignore_then(ident.clone())
            .then_ignore(just(Token::LBracket))
            .then(expr.clone())
            .then_ignore(just(Token::RBracket))
            .map(|(name, size)| Stmt::ArrayDecl { name, size });

        let decl = just(Token::Const)
            .or_not()
            .then_ignore(just(Token::IntType))
            .then(ident.clone())
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|((is_const, name), value)| Stmt::Declaration {
                name,
                is_mutable: is_const.is_none(),
                value,
            });

        let assign = ident
            .clone()
            .then(
                just(Token::LBracket)
                    .ignore_then(expr.clone())
                    .then_ignore(just(Token::RBracket))
                    .or_not(),
            )
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|((name, index), value)| Stmt::Assignment { name, index, value });

        let print = just(Token::Print)
            .ignore_then(expr.clone())
            .map(Stmt::Print);

        let brk = just(Token::Break).to(Stmt::Break);

        let get = just(Token::Get)
            .ignore_then(ident.clone())
            .then(
                just(Token::LBracket)
                    .ignore_then(expr.clone())
                    .then_ignore(just(Token::RBracket))
                    .or_not(),
            )
            .map(|(name, index)| Stmt::Get { name, index });

        let repeat = just(Token::Repeat)
            .ignore_then(stmt.clone().repeated().collect::<Vec<Stmt>>())
            .then_ignore(just(Token::End))
            .map(|body| Stmt::Repeat { body });

        let if_stmt = just(Token::If)
            .ignore_then(expr.clone())
            .then(stmt.clone().repeated().collect::<Vec<Stmt>>())
            .then(
                just(Token::Else)
                    .ignore_then(stmt.clone().repeated().collect())
                    .or_not(),
            )
            .then_ignore(just(Token::End))
            .map(|((condition, then_body), else_body)| Stmt::If {
                condition,
                then_body,
                else_body,
            });

        arr_decl
            .or(decl)
            .or(assign)
            .or(print)
            .or(get)
            .or(repeat)
            .or(if_stmt)
            .or(brk)
    })
    .repeated()
    .collect::<Vec<Stmt>>()
}
