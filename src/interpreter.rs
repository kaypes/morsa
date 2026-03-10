use crate::ast::{Expr, Stmt};
use std::collections::HashMap;
use std::io::{self, Write};

pub enum Flow {
    None,
    Break,
}
pub struct Symbol {
    pub value: i32,
    pub is_mutable: bool,
}
pub struct Environment {
    pub memory: HashMap<String, Symbol>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn eval(&self, expr: &Expr) -> Result<i32, String> {
        match expr {
            Expr::Int(n) => Ok(*n),
            Expr::Identifier(s) => self
                .memory
                .get(s)
                .map(|sym| sym.value)
                .ok_or(format!("Variavel '{}' nao definida", s)),
            Expr::Add(a, b) => Ok(self.eval(a)? + self.eval(b)?),

            Expr::Sub(a, b) => Ok(self.eval(a)? - self.eval(b)?),
            Expr::Mul(a, b) => Ok(self.eval(a)? * self.eval(b)?),
            Expr::Div(a, b) => {
                let den = self.eval(b)?;
                if den == 0 {
                    Err("A Morsa odeia divisao por zero!".into())
                } else {
                    Ok(self.eval(a)? / den)
                }
            }

            Expr::Eq(a, b) => Ok(if self.eval(a)? == self.eval(b)? { 1 } else { 0 }),
            Expr::Less(a, b) => Ok(if self.eval(a)? < self.eval(b)? { 1 } else { 0 }),
        }
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) -> Result<Flow, String> {
        for stmt in stmts {
            match stmt {
                Stmt::Break => return Ok(Flow::Break),
                Stmt::Declaration {
                    name,
                    is_mutable,
                    value,
                } => {
                    let v = self.eval(&value)?;
                    self.memory.insert(
                        name,
                        Symbol {
                            value: v,
                            is_mutable,
                        },
                    );
                }
                Stmt::Assignment { name, value } => {
                    let v = self.eval(&value)?;
                    let sym = self.memory.get_mut(&name).ok_or("Inexistente")?;
                    if !sym.is_mutable {
                        return Err("Imutavel".into());
                    }
                    sym.value = v;
                }
                Stmt::Print(e) => println!("{}", self.eval(&e)?),
                Stmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    if self.eval(&condition)? != 0 {
                        if let Flow::Break = self.execute(then_body)? {
                            return Ok(Flow::Break);
                        }
                    } else if let Some(e_body) = else_body {
                        if let Flow::Break = self.execute(e_body)? {
                            return Ok(Flow::Break);
                        }
                    }
                }
                Stmt::Repeat { body } => loop {
                    if let Flow::Break = self.execute(body.clone())? {
                        break;
                    }
                },

                Stmt::Get { name } => {
                    // Pede o número no terminal
                    print!("🦭 Digite um numero para {}: ", name);
                    io::stdout().flush().unwrap(); // Força o print a aparecer na hora

                    // Lê a linha digitada
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    // Converte para i32 (se o usuário digitar besteira, vira 0)
                    let v = input.trim().parse::<i32>().unwrap_or(0);

                    // Salva na variável (se ela existir e for mutável)
                    let sym = self
                        .memory
                        .get_mut(&name)
                        .ok_or(format!("Variavel '{}' nao existe", name))?;
                    if !sym.is_mutable {
                        return Err(format!("'{}' e imutavel (CONST)", name));
                    }
                    sym.value = v;
                }
            }
        }
        Ok(Flow::None)
    }
}
