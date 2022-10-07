use logos::{Logos};
use crate::position::*;
use crate::error::*;
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\r\f]+")]
    WS,
    #[error]
    Error,
    #[regex(r"\n+")]
    NL,
    EOF,

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[token("=")]
    Equal,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token("+")]
    Add,
    #[token("-")]
    Subtract,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("^")]
    Power,
    #[token("%")]
    Modulo,
    #[token("#")]
    Hashtag,
    
    #[token("(")]
    GroupIn,
    #[token(")")]
    GroupOut,
    #[token("[")]
    VectorIn,
    #[token("]")]
    VectorOut,
    #[token("{")]
    BraceIn,
    #[token("}")]
    BraceOut,

    #[regex(r"is")]
    TypeEq,

    #[regex(r"inf|infinity")]
    Infinity,
    #[regex(r"pi")]
    PI,
    #[regex(r"[a-zA-Z][a-zA-Z_0-9]*", |lex| lex.slice().to_string())]
    Variable(String),
}
impl Token {
    pub fn name(&self) -> &str {
        match &self {
            Self::WS => "white space",
            Self::Error => "error",
            Self::NL => "end of line",
            Self::EOF => "end of file",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Infinity => "infinity",
            Self::PI => "pi",
            Self::Variable(_) => "variable",
            Self::Equal => "'='",
            Self::Greater => "'>'",
            Self::Less => "'<'",
            Self::Add => "'+'",
            Self::Subtract => "'-'",
            Self::Multiply => "'*'",
            Self::Divide => "'/'",
            Self::Power => "'^'",
            Self::Modulo => "'%'",
            Self::Hashtag => "'#'",
            Self::GroupIn => "'('",
            Self::GroupOut => "')'",
            Self::VectorIn => "'['",
            Self::VectorOut => "']'",
            Self::BraceIn => "'{'",
            Self::BraceOut => "'}'",
            Self::TypeEq => "'is'",
        }
    }
}

pub fn lex(text: &str, file_path: &str) -> Result<Vec<(Token, Position)>, Error> {
    let mut lex = Token::lexer(text);
    let mut tokens: Vec<(Token, Position)> = vec![];
    let mut line: usize = 0;
    let mut column: usize = 0;
    loop {
        match lex.next() {
            Some(token) => {
                if token == Token::Error {
                    return Err(Error::Syntax(
                        format!("bad character '{}'", lex.slice()),
                        Position::new(lex.span().start, lex.span().end, line, line+1, column, column+1),
                        file_path.to_string()
                    ))
                }
                if token == Token::WS {
                    column += lex.span().len();
                    continue;
                }
                let (line_start, column_start) = (line, column);
                if token == Token::NL {
                    line += lex.span().len();
                    column = 0;
                }
                column += lex.span().len();
                tokens.push((token,
                    Position::new(lex.span().start, lex.span().end, line_start, line, column_start, column)
                ));
            }
            None => {
                tokens.push((Token::EOF,
                    Position::new(lex.span().start, lex.span().end, line, line, column, column)
                ));
                break
            }
        };
    }
    Ok(tokens)
}