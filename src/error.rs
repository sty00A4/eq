use std::fmt::{Debug, Display, Formatter, Error as FMTError};
use crate::position::*;
use crate::lexer::*;
use crate::parser::*;
use crate::interpreter::*;
#[derive(Clone, PartialEq)]
pub enum Error {
    Syntax(String, Position, String),
    ExpectToken(Token, Token, Position, String),
    ExpectNode(Node, Node, Position, String),
    UnexpectedToken(Token, Position, String),
    BinaryOperation(Token, Value, Value, Position, String),
    UnaryOperation(Token, Value, Position, String),
    Index(usize, usize, Position, String),
    IllegalValue(Value, Type, Position, String),
    Variable(String, Position, String),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match &self {
            Self::Syntax(detail, pos, path) =>
            write!(f, "ERROR: {detail} - {path} {pos}"),
            Self::ExpectToken(token1, token2, pos, path) =>
            write!(f, "ERROR: expected {} got {} - {path} {pos}", token1.name(), token2.name()),
            Self::ExpectNode(node1, node2, pos, path) =>
            write!(f, "ERROR: expected {node1} got {node2} - {path} {pos}"),
            Self::UnexpectedToken(token, pos, path) =>
            write!(f, "ERROR: unexpected {} - {path} {pos}", token.name()),
            Self::BinaryOperation(op, left, right, pos, path) =>
            write!(f, "ERROR: operation {} cannot be performed on {} and {} - {path} {pos}",
            op.name(), left.type_(), right.type_()),
            Self::UnaryOperation(op, value, pos, path) =>
            write!(f, "ERROR: operation {} cannot be performed on {} - {path} {pos}",
            op.name(), value.type_()),
            Self::Index(vector_len, index, pos, path) =>
            write!(f, "ERROR: index {index} out of range, max {vector_len} - {path} {pos}"),
            Self::IllegalValue(value, typ, pos, path) =>
            write!(f, "ERROR: {} illegal for {typ} - {path} {pos}", value.type_()),
            Self::Variable(var, pos, path) =>
            write!(f, "ERROR: {var} not defined - {path} {pos}"),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{self}")
    }
}