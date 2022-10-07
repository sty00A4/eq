use std::cmp::min;
use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use std::{fs, vec};
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

pub fn binary(op: &Token, left: &Value, right: &Value, context: &mut Context) -> Result<Value, ()> {
    match op {
        Token::Add => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int(v1 + v2)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 + v2)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Float(v1 + *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 + v2)),
            (Value::Vector(v1), Value::Int(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    vector.push(value);
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
            (Value::Vector(v1), Value::Int(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    vector.push(value);
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
            (Value::Vector(v1), Value::Int(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Float(_)) => {
                let mut vector: Vec<Value> = vec![];
                for v in v1 {
                    let value = binary(op, v, right, context)?;
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut vector: Vec<Value> = vec![];
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    vector.push(value);
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
                (Value::Vector(v1), Value::Int(_)) => {
                    let mut vector: Vec<Value> = vec![];
                    for v in v1 {
                        let value = binary(op, v, right, context)?;
                        vector.push(value);
                    }
                    Ok(Value::Vector(vector))
                }
                (Value::Vector(v1), Value::Float(_)) => {
                    let mut vector: Vec<Value> = vec![];
                    for v in v1 {
                        let value = binary(op, v, right, context)?;
                        vector.push(value);
                    }
                    Ok(Value::Vector(vector))
                }
                (Value::Vector(v1), Value::Vector(v2)) => {
                    let mut vector: Vec<Value> = vec![];
                    for i in 0..min(v1.len(), v2.len()) {
                        let value = binary(op, &v1[i], &v2[i], context)?;
                        vector.push(value);
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
        Token::Equal => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 == v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int((*v1 as f64 == *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 == *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 == v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                if v1.len() != v2.len() { return Ok(Value::Int(0)) }
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(equal as i64))
            }
            _ => Err(())
        }
        Token::NotEqual => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 != v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int((*v1 as f64 != *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 != *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 != v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                if v1.len() != v2.len() { return Ok(Value::Int(1)) }
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(&Token::Equal, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(!equal as i64))
            }
            _ => Err(())
        }
        Token::Less => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 < v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int(((*v1 as f64) < *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 < *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 < v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(equal as i64))
            }
            _ => Err(())
        }
        Token::Greater => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 > v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int((*v1 as f64 > *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 > *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 > v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(equal as i64))
            }
            _ => Err(())
        }
        Token::LessEqual => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 <= v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int(((*v1 as f64) <= *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 <= *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 <= v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(equal as i64))
            }
            _ => Err(())
        }
        Token::GreaterEqual => match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Ok(Value::Int((v1 >= v2) as i64)),
            (Value::Int(v1), Value::Float(v2)) => Ok(Value::Int((*v1 as f64 >= *v2) as i64)),
            (Value::Float(v1), Value::Int(v2)) => Ok(Value::Int((*v1 >= *v2 as f64) as i64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Int((v1 >= v2) as i64)),
            (Value::Vector(v1), Value::Vector(v2)) => {
                let mut equal = true;
                for i in 0..min(v1.len(), v2.len()) {
                    let value = binary(op, &v1[i], &v2[i], context)?;
                    if let Value::Int(int) = value { equal = int != 0; }
                    if !equal { break }
                }
                Ok(Value::Int(equal as i64))
            }
            _ => Err(())
        }
        _ => Err(())
    }
}
pub fn unary(op: &Token, value: &Value, context: &mut Context) -> Result<Value, ()> {
    match op {
        Token::Subtract => match &value {
            Value::Int(v) => Ok(Value::Int(-v)),
            Value::Float(v) => Ok(Value::Float(-v)),
            Value::Vector(v) => {
                let mut vector: Vec<Value> = vec![];
                for value in v {
                    vector.push(unary(op, value, context)?);
                }
                Ok(Value::Vector(vector))
            }
        }
        _ => Err(())
    }
}

pub fn interpret(node_and_pos: &(Node, Position), file_path: &str, context: &mut Context) -> Result<Value, Error> {
    let (node, node_pos) = node_and_pos;
    match node {
        Node::Int(int) => Ok(Value::Int(*int)),
        Node::Float(float) => Ok(Value::Float(*float)),
        Node::Infinity => Ok(Value::Float(INFINITY)),
        Node::PI => Ok(Value::Float(PI)),
        Node::Variable(var) => {
            let value = context.get(var);
            if value.is_none() { return Err(Error::Variable(var.clone(), node_pos.clone(), file_path.to_string())) }
            return Ok(value.unwrap())
        }
        Node::Set(var_node, expr_node) => {
            let value = interpret(&(expr_node.0.as_ref().clone(), expr_node.1.clone()), file_path, context)?;
            if let Node::Variable(var) = var_node.0.as_ref() {
                context.set(var, &value);
                return Ok(value)
            }
            Err(Error::ExpectNode(Node::Variable(
                "".to_string()), var_node.0.as_ref().clone(), var_node.1.clone(), file_path.to_string()
            ))
        }
        Node::Vector(nodes) => {
            let mut vector: Vec<Value> = vec![];
            for n in nodes {
                let value = interpret(&n, file_path, context)?;
                match &value {
                    Value::Int(_) => {},
                    Value::Float(_) => {},
                    _ => return Err(Error::IllegalValue(value, Type::Vector, n.1.clone(), file_path.to_string()))
                }
                vector.push(value);
            }
            Ok(Value::Vector(vector))
        }
        Node::Binary(op, left_node, right_node) => {
            let left = interpret(&(left_node.0.as_ref().clone(), left_node.1.clone()), file_path, context)?;
            let right = interpret(&(right_node.0.as_ref().clone(), right_node.1.clone()), file_path, context)?;
            let res = binary(op, &left, &right, context);
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
            let value = interpret(&(n.0.as_ref().clone(), n.1.clone()), file_path, context)?;
            let res = unary(op, &value, context);
            if res.is_err() { return Err(Error::UnaryOperation(
                op.clone(), value, node_pos.clone(), file_path.to_string()
            )) }
            Ok(res.unwrap())
        }
    }
}

pub struct Context {
    pub stack: Vec<(String, Value)>
}
impl Context {
    pub fn new() -> Self { Self { stack: vec![] } }
    pub fn set(&mut self, var: &String, value: &Value) {
        for reg in self.stack.iter_mut() {
            if &reg.0 == var { reg.1 = value.clone(); return }
        }
        self.stack.push((var.clone(), value.clone()));
    }
    pub fn get(&mut self, var: &String) -> Option<Value> {
        for reg in self.stack.iter() {
            if &reg.0 == var { return Some(reg.1.clone()) }
        }
        None
    }
}

pub fn run(text: &str, file_path: &str, context: &mut Context) -> Option<Value> {
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

    let res = interpret(&(node, pos), file_path, context);
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
    let mut context = Context::new();
    run(text.as_str(), file_path, &mut context);
}