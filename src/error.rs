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
    NotImplemented(String, Position, String),
    BinaryOperation(Token, Value, Value, Position, String),
    UnaryOperation(Token, Value, Position, String),
    Index(usize, usize, Position, String),
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
            Self::Index(vector_len, index, pos, path) =>
            write!(f, "ERROR: index {index} out of range, max {vector_len} - {path} {pos}"),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        write!(f, "{self}")
    }
}