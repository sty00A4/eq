// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_must_use)]

extern crate logos;
use std::io::Write;
use std::{env, io};

mod error;
mod position;
mod lexer;
mod parser;
mod interpreter;
use crate::interpreter::*;

// -- INTERPRET ------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        runfile(args[1].as_str());
        return
    }
    let mut context = Context::new();
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        let value = run(input.as_str(), "<shell>", &mut context);
        if let Some(v) = value { println!("{v}") }
    }
}
