use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::{Stmt, Stmt::*};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Expression { expression } => {
                    let _ = expression.evaluate(&self.environment);
                }
                Print { expression } => {
                    let value = expression.evaluate(&self.environment);

                    println!("{value:?}")
                }
                Var { name, initializer } => {
                    let value = initializer.evaluate(&self.environment)?;

                    self.environment.define(name.lexeme, value)
                }
            }
        }

        Ok(())
    }
}
