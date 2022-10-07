use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use crate::position::*;
use crate::error::*;
use crate::lexer::*;
#[derive(Clone, PartialEq)]
pub enum Node {
    Binary(Token, (Box<Node>, Position), (Box<Node>, Position)), Unary(Token, (Box<Node>, Position)),
    Int(i64), Float(f64), Infinity, PI, Variable(String), Vector(Vec<(Node, Position)>)
}
// impl Node {
//     pub fn name(&self) -> &str {
//         match &self {
//             Self::Binary(token, _, _) => "binary operation",
//             Self::Unary(token, _) => "unary operation",
//             Self::Int(_) => "int",
//             Self::Float(_) => "float",
//             Self::Infinity => "infinity",
//             Self::PI => "pi",
//             Self::Variable(_) => "variable",
//             Self::Vector(_) => "vector",
//         }
//     }
// }
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Int(int) => write!(f, "({int})"),
            Self::Float(float) => write!(f, "({float})"),
            Self::PI => write!(f, "(pi)"),
            Self::Infinity => write!(f, "(inf)"),
            Self::Variable(var) => write!(f, "({var})"),
            Self::Vector(vector) => {
                let mut strings: Vec<String> = vec![];
                for (node, _) in vector {
                    strings.push(format!("{node}"));
                }
                write!(f, "[{}]", strings.join(" "))
            },
            Self::Binary(op, (left, _), (right, _)) => write!(f, "({left} {} {right})", op.name()),
            Self::Unary(op, (node, _)) => write!(f, "({} {node})", op.name()),
        }
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{self}")
    }
}

pub struct Parser {
    tokens: Vec<(Token, Position)>,
    idx: usize,
    file_path: String,
}
impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>, file_path: String) -> Self {
        Self { tokens, idx: 0, file_path }
    }
    pub fn token(&self) -> Token { self.tokens[self.idx].0.clone() }
    pub fn expect_token(&self, token: Token) -> Result<(), Error> {
        if token != self.token() { return Err(Error::ExpectToken(token, self.token(), self.pos(), self.file_path.clone())) }
        Ok(())
    }
    pub fn pos(&self) -> Position { self.tokens[self.idx].1.clone() }
    pub fn advance(&mut self) { self.idx += 1 }
    pub fn advance_nl(&mut self) { while self.token() == Token::NL { self.idx += 1 } }
    pub fn parse(&mut self) -> Result<(Node, Position), Error> {
        let node = self.expr()?;
        self.expect_token(Token::NL)?;
        self.advance_nl();
        self.expect_token(Token::EOF)?;
        Ok(node)
    }
    pub fn expr(&mut self) -> Result<(Node, Position), Error> {
        self.arith()
    }
    pub fn arith(&mut self) -> Result<(Node, Position), Error> {
        let (mut left, mut left_pos) = self.term()?;
        while [Token::Add, Token::Subtract].contains(&self.token()) {
            let op = self.token();
            self.advance();
            let (right, right_pos) = self.term()?;
            left = Node::Binary(op, (Box::new(left), left_pos.clone()), (Box::new(right), right_pos.clone()));
            left_pos.extend(right_pos);
        }
        Ok((left, left_pos))
    }
    pub fn term(&mut self) -> Result<(Node, Position), Error> {
        let (mut left, mut left_pos) = self.power()?;
        while [Token::Multiply, Token::Divide].contains(&self.token()) {
            let op = self.token();
            self.advance();
            let (right, right_pos) = self.power()?;
            left = Node::Binary(op, (Box::new(left), left_pos.clone()), (Box::new(right), right_pos.clone()));
            left_pos.extend(right_pos);
        }
        Ok((left, left_pos))
    }
    pub fn power(&mut self) -> Result<(Node, Position), Error> {
        let (mut left, mut left_pos) = self.factor()?;
        while self.token() == Token::Power {
            self.advance();
            let (right, right_pos) = self.factor()?;
            left = Node::Binary(Token::Power, (Box::new(left), left_pos.clone()), (Box::new(right), right_pos.clone()));
            left_pos.extend(right_pos);
        }
        Ok((left, left_pos))
    }
    pub fn factor(&mut self) -> Result<(Node, Position), Error> {
        let mut pos = self.pos();
        if self.token() == Token::Subtract {
            self.advance();
            let (node, node_pos) = self.factor()?;
            pos.extend(node_pos.clone());
            return Ok((Node::Unary(Token::Subtract,(Box::new(node), node_pos)), pos))
        }
        self.hash()
    }
    pub fn hash(&mut self) -> Result<(Node, Position), Error> {
        let (mut left, mut left_pos) = self.atom()?;
        while self.token() == Token::Hashtag {
            self.advance();
            let (right, right_pos) = self.atom()?;
            left = Node::Binary(Token::Hashtag, (Box::new(left), left_pos.clone()), (Box::new(right), right_pos.clone()));
            left_pos.extend(right_pos);
        }
        Ok((left, left_pos))
    }
    pub fn atom(&mut self) -> Result<(Node, Position), Error> {
        match self.token() {
            Token::Int(int) => {
                self.advance();
                Ok((Node::Int(int), self.pos()))
            }
            Token::Float(float) => {
                self.advance();
                Ok((Node::Float(float), self.pos()))
            }
            Token::Variable(var) => {
                self.advance();
                Ok((Node::Variable(var), self.pos()))
            }
            Token::Infinity => {
                self.advance();
                Ok((Node::Infinity, self.pos()))
            }
            Token::PI => {
                self.advance();
                Ok((Node::PI, self.pos()))
            }
            Token::GroupIn => {
                self.advance();
                let node = self.expr()?;
                self.expect_token(Token::GroupOut)?;
                self.advance();
                Ok(node)
            }
            Token::VectorIn => {
                let mut pos = self.pos();
                self.advance();
                let mut nodes: Vec<(Node, Position)> = vec![];
                while self.token() != Token::VectorOut {
                    let (node, node_pos) = self.expr()?;
                    pos.extend(node_pos.clone());
                    nodes.push((node, node_pos));
                }
                self.advance();
                pos.extend(self.pos());
                Ok((Node::Vector(nodes), pos))
            }
            _ => Err(Error::UnexpectedToken(self.token(), self.pos(), self.file_path.clone()))
        }
    }
}
pub fn parse(tokens: Vec<(Token, Position)>, file_path: &str) -> Result<(Node, Position), Error> {
    Parser::new(tokens, file_path.to_string()).parse()
}