use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::dodo::error_types::DodoParseError;
use crate::core::dodo::error_types::throw_error;
use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, Vec<i128>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: String, value: Vec<i128>) {
        println!("{}", &name);
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, values: Vec<i128>) {
        // todo handle different types
        if self.values.contains_key(&name.val) {
            self.values.insert(name.val, values);
        }
    }
    pub fn get(&mut self, name: Token) -> Option<&Vec<i128>> {
        if self.values.contains_key(&name.val) {
            return self.values.get(&name.val);
        }
        return None;
    }
}