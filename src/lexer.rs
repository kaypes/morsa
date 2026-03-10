use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq, Hash)]
#[logos(skip r"[ \t\n\f/]+")]
pub enum Token {
    #[token("-.-. --- -. ... -")]       Const,
    #[token(".--. .-. .. -. -")]        Print,
    #[token(".. -. -")]                 IntType,
    #[token("-...-")]                   Assign,
    #[token(".-.-.")]                   Plus,
    #[token("-....-")]                  Minus,
    #[token("-..-")]                    Star,
    #[token("-..-.")]                   Slash,
    #[token(".-. . .--. . .- -")]       Repeat,
    #[token(". -. -..")]                End,
    #[token(".. ..-.")]                 If,
    #[token(". .-.. ... .")]            Else,
    #[token("-... .-. . .- -.-")]       Break,
    #[token("-...- -...-")]             Eq,
    #[token(".-.. -")]                  Less,

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
