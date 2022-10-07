use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use std::fs;
use crate::error::*;
use crate::position::*;
use crate::lexer::*;
use crate::parser::*;
use std::f64::{consts::PI, INFINITY};
#[derive(Clone, PartialEq)]
pub enum Type {
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
pub enum Value {
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

pub fn binary(op: &Token, left_node: &(Node, Position), right_node: &(Node, Position), pos: Position, file_path: &str) -> Result<Value, Error> {
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
pub fn unary(op: &Token, node: &(Node, Position), pos: Position, file_path: &str) -> Result<Value, Error> {
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

pub fn interpret(node_and_pos: &(Node, Position), file_path: &str) -> Result<Value, Error> {
    let (node, node_pos) = node_and_pos;
    match node {
        Node::Int(int) => Ok(Value::Int(*int)),
        Node::Float(float) => Ok(Value::Float(*float)),
        Node::Infinity => Ok(Value::Float(INFINITY)),
        Node::PI => Ok(Value::Float(PI)),
        Node::Vector(nodes) => {
            let mut vector: Vec<Value> = vec![];
            for n in nodes {
                let value = interpret(&n, file_path)?;
                vector.push(value);
            }
            Ok(Value::Vector(vector))
        }
        Node::Binary(op, left, right) => binary(
            op, &(left.0.as_ref().clone(), left.1.clone()), &(right.0.as_ref().clone(), right.1.clone()),
            node_pos.clone(), file_path
        ),
        Node::Unary(op, n) => unary(
            op, &(n.0.as_ref().clone(), n.1.clone()),
            node_pos.clone(), file_path
        ),
        _ => Err(Error::NotImplemented(format!("{node:?}"), node_pos.clone(), file_path.to_string()))
    }
}

// -- RUNNING --------------------------------------------------------------------------
pub fn run(text: &str, file_path: &str) -> Option<Value> {
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
pub fn runfile(file_path: &str) {
    let res = fs::read_to_string(file_path);
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return
    }
    let text = res.unwrap();
    run(text.as_str(), file_path);
}