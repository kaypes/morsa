use crate::ast::{Expr, Stmt};
use std::collections::HashMap;
use std::io::{self, Write};

pub enum Flow {
    None,
    Break,
}

pub struct Symbol {
    pub values: Vec<i32>,
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
                .map(|sym| sym.values[0])
                .ok_or(format!("Variavel '{}' nao definida", s)),
            Expr::ArrayAccess(s, idx) => {
                let i = self.eval(idx)? as usize;
                let sym = self
                    .memory
                    .get(s)
                    .ok_or(format!("Array '{}' nao existe", s))?;
                sym.values
                    .get(i)
                    .copied()
                    .ok_or(format!("Indice {} fora do limite do array '{}'", i, s))
            }
            Expr::Add(a, b) => Ok(self.eval(a)? + self.eval(b)?),
            Expr::Sub(a, b) => Ok(self.eval(a)? - self.eval(b)?),
            Expr::Mul(a, b) => Ok(self.eval(a)? * self.eval(b)?),
            Expr::Div(a, b) => {
                let den = self.eval(b)?;
                if den == 0 {
                    Err("Divisao por zero!".into())
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
                            values: vec![v],
                            is_mutable,
                        },
                    );
                }

                Stmt::ArrayDecl { name, size } => {
                    let s = self.eval(&size)? as usize;
                    self.memory.insert(
                        name,
                        Symbol {
                            values: vec![0; s],
                            is_mutable: true,
                        },
                    );
                }

                Stmt::Assignment { name, index, value } => {
                    let v = self.eval(&value)?;

                    let idx_val = match index {
                        Some(expr) => Some(self.eval(&expr)? as usize),
                        None => None,
                    };

                    let sym = self.memory.get_mut(&name).ok_or("Inexistente")?;
                    if !sym.is_mutable {
                        return Err("Imutavel".into());
                    }

                    if let Some(i) = idx_val {
                        if i >= sym.values.len() {
                            return Err("Indice fora do limite".into());
                        }
                        sym.values[i] = v;
                    } else {
                        sym.values[0] = v;
                    }
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

                Stmt::Get { name, index } => {
                    print!("{}: ", name);
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let v = input.trim().parse::<i32>().unwrap_or(0);

                    let idx_val = match index {
                        Some(expr) => Some(self.eval(&expr)? as usize),
                        None => None,
                    };

                    let sym = self
                        .memory
                        .get_mut(&name)
                        .ok_or(format!("Variavel '{}' nao existe", name))?;
                    if !sym.is_mutable {
                        return Err("Imutavel (CONST)".into());
                    }

                    if let Some(i) = idx_val {
                        if i >= sym.values.len() {
                            return Err("Indice fora do limite".into());
                        }
                        sym.values[i] = v;
                    } else {
                        sym.values[0] = v;
                    }
                }
            }
        }
        Ok(Flow::None)
    }
}
