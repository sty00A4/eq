use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use std::{fs, vec};
use crate::error::*;
use crate::position::*;
use crate::lexer::*;
use crate::parser::*;
use std::f64::{consts::PI, INFINITY};
#[derive(Clone, PartialEq)]
pub enum Type {
    Int, Float, Vector, Type
}
impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Vector => write!(f, "vector"),
            Self::Type => write!(f, "type"),
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
    Int(i64), Float(f64), Vector(Vec<Value>), Type(Type)
}
impl Value {
    pub fn type_(&self) -> Type {
        match self {
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Vector(_) => Type::Vector,
            Self::Type(_) => Type::Type,
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Int(int) => write!(f, "{int}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Vector(vector) => write!(f, "{vector:?}"),
            Self::Type(typ) => write!(f, "{typ}"),
        }
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{}", self)
    }
}

pub fn binary(op: &Token, left: &Value, right: &Value) -> Result<Value, ()> {
    match op {
        Token::Add => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 + v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 + v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 + *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 + v2)),
            (Value::Vector(v1), Value::Int(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Int(int + v2)),
                        Value::Float(float) => vector.push(Value::Float(float + *v2 as f64)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Float(*int as f64 + v2)),
                        Value::Float(float) => vector.push(Value::Float(float + v2)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..v1.len() {
                    match v1[i] {
                        Value::Int(int1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Int(int1 + int2)),
                            Value::Float(float2) => vector.push(Value::Float(int1 as f64 + float2)),
                            _ => return Err(())
                        },
                        Value::Float(float1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Float(float1 + int2 as f64)),
                            Value::Float(float2) => vector.push(Value::Float(float1 + float2)),
                            _ => return Err(())
                        },
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            _ => Err(())
        }
        Token::Subtract => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 - v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 - v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 - *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 - v2)),
            (Value::Vector(v1), Value::Int(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Int(int - v2)),
                        Value::Float(float) => vector.push(Value::Float(float - *v2 as f64)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Float(*int as f64 - v2)),
                        Value::Float(float) => vector.push(Value::Float(float - v2)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..v1.len() {
                    match v1[i] {
                        Value::Int(int1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Int(int1 - int2)),
                            Value::Float(float2) => vector.push(Value::Float(int1 as f64 - float2)),
                            _ => return Err(())
                        },
                        Value::Float(float1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Float(float1 - int2 as f64)),
                            Value::Float(float2) => vector.push(Value::Float(float1 -float2)),
                            _ => return Err(())
                        },
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            _ => Err(())
        }
        Token::Multiply => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 * v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 * v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 * *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 * v2)),
            (Value::Vector(v1), Value::Int(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Int(int * v2)),
                        Value::Float(float) => vector.push(Value::Float(float * *v2 as f64)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    match v {
                        Value::Int(int) => vector.push(Value::Float(*int as f64 * v2)),
                        Value::Float(float) => vector.push(Value::Float(float * v2)),
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..v1.len() {
                    match v1[i] {
                        Value::Int(int1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Int(int1 * int2)),
                            Value::Float(float2) => vector.push(Value::Float(int1 as f64 * float2)),
                            _ => return Err(())
                        },
                        Value::Float(float1) => match v2[i] {
                            Value::Int(int2) => vector.push(Value::Float(float1 * int2 as f64)),
                            Value::Float(float2) => vector.push(Value::Float(float1 * float2)),
                            _ => return Err(())
                        },
                        _ => return Err(())
                    };
                }
                Ok(Value::Vector(vector))
            }
            _ => Err(())
        }
        Token::Divide => {
            if (left, right) == (&Value::Float(INFINITY), &Value::Float(INFINITY)) {
                return Ok(Value::Float(INFINITY))
            }
            match (left, right) {
                (Value::Int(v1), Value::Int(v2)) => Ok(Value::Float(*v1 as f64 / *v2 as f64)),
                (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 / v2)),
                (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 / *v2 as f64)),
                (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 / v2)),
                (Value::Vector(v1), Value::Int(v2)) => {
                    let mut vector: Vec<Value> = vec![];
                    for v in v1 {
                        match v {
                            Value::Int(int) => vector.push(Value::Float(*int as f64 / *v2 as f64)),
                            Value::Float(float) => vector.push(Value::Float(float / *v2 as f64)),
                            _ => return Err(())
                        };
                    }
                    Ok(Value::Vector(vector))
                }
                (Value::Vector(v1), Value::Float(v2)) => {
                    let mut vector: Vec<Value> = vec![];
                    for v in v1 {
                        match v {
                            Value::Int(int) => vector.push(Value::Float(*int as f64 / v2)),
                            Value::Float(float) => vector.push(Value::Float(float / v2)),
                            _ => return Err(())
                        };
                    }
                    Ok(Value::Vector(vector))
                }
                (Value::Vector(v1), Value::Vector(v2)) => {
                    let mut vector: Vec<Value> = vec![];
                    for i in 0..v1.len() {
                        match v1[i] {
                            Value::Int(int1) => match v2[i] {
                                Value::Int(int2) => vector.push(Value::Float(int1 as f64 / int2 as f64)),
                                Value::Float(float2) => vector.push(Value::Float(int1 as f64 / float2)),
                                _ => return Err(())
                            },
                            Value::Float(float1) => match v2[i] {
                                Value::Int(int2) => vector.push(Value::Float(float1 / int2 as f64)),
                                Value::Float(float2) => vector.push(Value::Float(float1 / float2)),
                                _ => return Err(())
                            },
                            _ => return Err(())
                        };
                    }
                    Ok(Value::Vector(vector))
                }
                _ => Err(())
            }
        }
        Token::Hashtag => match (left, right) {
            (Value::Vector(vector), Value::Int(index)) => {
                if (*index as usize) >= vector.len() { return Err(()) }
                Ok(vector[*index as usize].clone())
            }
            _ => Err(())
        }
        _ => Err(())
    }
}
pub fn unary(op: &Token, value: &Value) -> Result<Value, ()> {
    match op {
        Token::Subtract => match &value {
            Value::Int(v) => Ok(Value::Int(-v)),
            Value::Float(v) => Ok(Value::Float(-v)),
            _ => Err(())
        }
        _ => Err(())
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
        Node::Binary(op, left_node, right_node) => {
            let left = interpret(&(left_node.0.as_ref().clone(), left_node.1.clone()), file_path)?;
            let right = interpret(&(right_node.0.as_ref().clone(), right_node.1.clone()), file_path)?;
            let res = binary(op, &left, &right);
            if res.is_err() {
                if op == &Token::Hashtag {
                    if let (Value::Vector(vector), Value::Int(index)) = (&left, &right) {
                        return Err(Error::Index(vector.len()-1, *index as usize, node_pos.clone(), file_path.to_string()))
                    }
                }
                return Err(Error::BinaryOperation(
                    op.clone(), left, right, node_pos.clone(), file_path.to_string()
                ))
            }
            Ok(res.unwrap())
        }
        Node::Unary(op, n) => {
            let value = interpret(&(n.0.as_ref().clone(), n.1.clone()), file_path)?;
            let res = unary(op, &value);
            if res.is_err() { return Err(Error::UnaryOperation(
                op.clone(), value, node_pos.clone(), file_path.to_string()
            )) }
            Ok(res.unwrap())
        }
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