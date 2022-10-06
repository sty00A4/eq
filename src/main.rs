#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

extern crate logos;
use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use std::io::Write;
use logos::{Logos};
use std::f64::{consts::PI, INFINITY};
use std::{env, io, fs};

// -- ERROR ----------------------------------------------------------------------------
enum Error {
    Syntax(String, Position, String),
    ExpectToken(Token, Token, Position, String),
    ExpectNode(Node, Node, Position, String),
    UnexpectedToken(Token, Position, String),
    NotImplemented(String, Position, String),
    BinaryOperation(Token, Value, Value, Position, String),
    UnaryOperation(Token, Value, Position, String),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Syntax(detail, pos, path) =>
            write!(f, "ERROR: {detail} - {path} {pos}"),
            Self::ExpectToken(token1, token2, pos, path) =>
            write!(f, "ERROR: expected {} got {} - {path} {pos}", token1.name(), token2.name()),
            Self::ExpectNode(node1, node2, pos, path) =>
            write!(f, "ERROR: expected {} got {} - {path} {pos}", node1.name(), node2.name()),
            Self::UnexpectedToken(token, pos, path) =>
            write!(f, "ERROR: unexpected {} - {path} {pos}", token.name()),
            Self::NotImplemented(msg, pos, path) =>
            write!(f, "ERROR: not implemented -> {msg} - {path} {pos}"),
            Self::BinaryOperation(op, left, right, pos, path) =>
            write!(f, "ERROR: operation {} cannot be performed on {} and {} - {path} {pos}",
            op.name(), left.type_(), right.type_()),
            Self::UnaryOperation(op, value, pos, path) =>
            write!(f, "ERROR: operation {} cannot be performed on {} - {path} {pos}",
            op.name(), value.type_()),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{self}")
    }
}

// -- POSITION -------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
struct Position {
    start: usize,
    end: usize,
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
}
impl Position {
    pub fn new(start: usize, end: usize, line_start: usize, line_end: usize, column_start: usize, column_end: usize) -> Self {
        Self { start, end, line_start, line_end, column_start, column_end }
    }
    pub fn extend(&mut self, pos: Position) {
        if pos.line_end > self.line_end { self.line_end = pos.line_end; }
        if pos.column_end > self.column_end { self.column_end = pos.column_end; }
        if pos.end > self.end { self.end = pos.end; }
    }
}
impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "<ln: {}, column: {}>", self.line_start, self.column_start)
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{:?}", self)
    }
}

// -- LEXING ---------------------------------------------------------------------------
#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
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
    #[regex(r"inf|infinity")]
    Infinity,
    #[regex(r"pi")]
    PI,
    #[regex(r"[a-zA-Z][a-zA-Z_0-9]*", |lex| lex.slice().to_string())]
    Variable(String),

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
            Self::GroupIn => "'('",
            Self::GroupOut => "')'",
            Self::VectorIn => "'['",
            Self::VectorOut => "']'",
            Self::BraceIn => "'{'",
            Self::BraceOut => "'}'",
        }
    }
}

fn lex(text: &str, file_path: &str) -> Result<Vec<(Token, Position)>, Error> {
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

// -- PARSING --------------------------------------------------------------------------
#[derive(Debug, Clone)]
enum Node {
    NO,
    Binary(Token, (Box<Node>, Position), (Box<Node>, Position)), Unary(Token, (Box<Node>, Position)),
    Int(i64), Float(f64), Infinity, PI, Variable(String), Vector(Vec<(Node, Position)>)
}
impl Node {
    pub fn name(&self) -> &str {
        match &self {
            Self::Binary(token, _, _) => "binary operation",
            Self::Unary(token, _) => "unary operation",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Infinity => "infinity",
            Self::PI => "pi",
            Self::Variable(_) => "variable",
            Self::Vector(_) => "vector",
            Self::NO => "nothing",
        }
    }
}

struct Parser {
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
            let (node, node_pos) = self.atom()?;
            pos.extend(node_pos.clone());
            return Ok((Node::Unary(Token::Subtract,(Box::new(node), node_pos)), pos))
        }
        self.atom()
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
                self.expect_token(Token::GroupOut);
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
fn parse(tokens: Vec<(Token, Position)>, file_path: &str) -> Result<(Node, Position), Error> {
    Parser::new(tokens, file_path.to_string()).parse()
}

// -- INTERPRET ------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum Type {
    Int, Float, Vector
}
impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Vector => write!(f, "vector"),
        }
    }
}
impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{}", self)
    }
}
#[derive(Clone, PartialEq)]
enum Value {
    Int(i64), Float(f64), Vector(Vec<Value>)
}
impl Value {
    pub fn type_(&self) -> Type {
        match self {
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Vector(_) => Type::Vector,
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Int(int) => write!(f, "{int}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Vector(vector) => write!(f, "{vector:?}"),
        }
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{}", self)
    }
}

fn binary(op: &Token, left_node: &(Node, Position), right_node: &(Node, Position), pos: Position, file_path: &str) -> Result<Value, Error> {
    let left = interpret(left_node, file_path)?;
    let right = interpret(right_node, file_path)?;
    match op {
        Token::Add => match (&left, &right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 + v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 + v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 + *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 + v2)),
            _ => Err(Error::BinaryOperation(op.clone(), left.clone(), right.clone(), pos.clone(), file_path.to_string()))
        }
        Token::Subtract => match (&left, &right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 - v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 - v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 - *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 - v2)),
            _ => Err(Error::BinaryOperation(op.clone(), left.clone(), right.clone(), pos.clone(), file_path.to_string()))
        }
        Token::Multiply => match (&left, &right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 * v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 * v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 * *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 * v2)),
            _ => Err(Error::BinaryOperation(op.clone(), left.clone(), right.clone(), pos.clone(), file_path.to_string()))
        }
        Token::Divide => {
            if (&left, &right) == (&Value::Float(INFINITY), &Value::Float(INFINITY)) {
                return Ok(Value::Float(INFINITY))
            }
            match (&left, &right) {
                (Value::Int(v1), Value::Int(v2)) => Ok(Value::Float(*v1 as f64 / *v2 as f64)),
                (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 / v2)),
                (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 / *v2 as f64)),
                (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 / v2)),
                _ => Err(Error::BinaryOperation(op.clone(), left.clone(), right.clone(), pos.clone(), file_path.to_string()))
            }
        }
        _ => Err(Error::NotImplemented(format!("{op:?}"), pos.clone(), file_path.to_string()))
    }
}
fn unary(op: &Token, node: &(Node, Position), pos: Position, file_path: &str) -> Result<Value, Error> {
    let value = interpret(node, file_path)?;
    match op {
        Token::Subtract => match &value {
            Value::Int(v) => Ok(Value::Int(-v)),
            Value::Float(v) => Ok(Value::Float(-v)),
            _ => Err(Error::UnaryOperation(op.clone(), value.clone(), pos.clone(), file_path.to_string()))
        }
        _ => Err(Error::NotImplemented(format!("{op:?}"), pos.clone(), file_path.to_string()))
    }
}

fn interpret(node_and_pos: &(Node, Position), file_path: &str) -> Result<Value, Error> {
    let (node, node_pos) = node_and_pos;
    match node {
        Node::Int(int) => Ok(Value::Int(*int)),
        Node::Float(float) => Ok(Value::Float(*float)),
        Node::Infinity => Ok(Value::Float(INFINITY)),
        Node::PI => Ok(Value::Float(PI)),
        Node::Binary(op, left, right) => binary(
            op, &(left.0.as_ref().clone(), left.1.clone()), &(right.0.as_ref().clone(), right.1.clone()),
            node_pos.clone(), file_path
        ),
        _ => Err(Error::NotImplemented(format!("{node:?}"), node_pos.clone(), file_path.to_string()))
    }
}

// -- RUNNING --------------------------------------------------------------------------
fn run(text: &str, file_path: &str) -> Option<Value> {
    let res = lex(text, file_path);
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return None
    }
    let tokens = res.unwrap();
    // println!("{tokens:?}");

    let res = parse(tokens, file_path);
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return None
    }
    let (node, pos) = res.unwrap();
    // println!("{node:?}");

    let res = interpret(&(node, pos), file_path);
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return None
    }
    let value = res.unwrap();
    return Some(value)
}
fn runfile(file_path: &str) {
    let res = fs::read_to_string(file_path);
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return
    }
    let text = res.unwrap();
    run(text.as_str(), file_path);
}
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        runfile(args[1].as_str());
        return
    }
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        let value = run(input.as_str(), "<shell>");
        if let Some(v) = value {
            println!("{v}")
        }
    }
}
