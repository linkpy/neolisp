#![allow(dead_code)]

#[macro_use]
extern crate peg;

mod nl;

use crate::nl::interpreter::builtin::register_all_builtin_forms;
use nl::core::object::*;
use nl::interpreter::*;
use nl::reader::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    repl();
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let mut scope = Scope::new();
    register_all_builtin_forms(&mut scope);

    loop {
        match rl.readline("> ") {
            Ok(line) => evaluate_line(&mut scope, line),
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn evaluate_line(scope: &mut Scope, line: String) {
    let parse_result = nl_parser::expr(&line);

    match parse_result {
        Ok(mut object) => {
            object.complete_location(&"REPL".to_string(), &line);

            match evaluate(scope, &object) {
                Ok(result) => println!("=> {}", result),
                Err(err) => println!("{}", err.to_string()),
            }
        }
        Err(err) => println!("Parse error: {:?}", err),
    }
}
