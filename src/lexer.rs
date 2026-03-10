use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq, Hash)]
#[logos(skip r"[ \t\n\f/]+")]
pub enum Token {
    #[token("-.-. --- -. ... -")]       Const,
    #[token(".. -. -")]                 IntType,
    #[token(".--. .-. .. -. -")]        Print,
    #[token("--. . -")]                 Get,
    #[token(".. ..-.")]                 If,
    #[token(". .-.. ... .")]            Else,
    #[token(".-. . .--. . .- -")]       Repeat,
    #[token("-... .-. . .- -.-")]       Break,
    #[token(". -. -..")]                End,
    #[token(".- -. -..")]               And,
    #[token("--- .-.")]                 Or,
    #[token("-...-")]                   Assign,
    #[token("-...- -...-")]             Eq,
    #[token("--. - -...-")]             GreaterEq,
    #[token("--. -")]                   Greater,
    #[token(".-.. -")]                  Less,
    #[token(".-.. - -...-")]            LessEq,
    #[token("-.--.")]                   LBracket,
    #[token("-.--.-")]                  RBracket,
    #[token(".-.-.")]                   Plus,
    #[token("-....-")]                  Minus,
    #[token("-..-")]                    Star,
    #[token("-..-.")]                   Slash,

    #[regex(r"[\.-]+", |lex| lex.slice().to_string())]
    Morse(String),
}

impl Token {
    pub fn to_digit(&self) -> Option<i32> {
        if let Token::Morse(s) = self {
            match s.as_str() {
                "-----" => Some(0),
                ".----" => Some(1),
                "..---" => Some(2),
                "...--" => Some(3),
                "....-" => Some(4),
                "....." => Some(5),
                "-...." => Some(6),
                "--..." => Some(7),
                "---.." => Some(8),
                "----." => Some(9),
                _ => None,
            }
        } else {
            None
        }
    }
}
