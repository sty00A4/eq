use std::fmt::{Debug, Display, Formatter, Error as FMTError};
#[derive(Clone, PartialEq)]
pub struct Position {
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