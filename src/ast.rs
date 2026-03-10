#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Identifier(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declaration {
        name: String,
        is_mutable: bool,
        value: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Repeat {
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    Break,
    Get {
        name: String,
    },
}
