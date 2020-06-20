use crate::core::ast::stmt::Stmt;


pub struct Program {
    pub statements: Vec<Stmt>
}

impl Program {
    pub fn new(program: Vec<Stmt>) -> Self {
        Program {
            statements: program
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic_statements() {
        let input = "scalar x
                           vector y[4]";
    }
}