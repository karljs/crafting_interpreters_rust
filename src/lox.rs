use crate::environment::Environment;
use crate::error::Result;
use crate::eval::Eval;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program::Program;

pub struct Lox {
    program: Program,
    environment: Environment,
}

impl Lox {
    pub fn new_from_input(input: String) -> Result<Self> {
        let lexer = Lexer::from_source(input);
        let mut parser = Parser::from_tokens(lexer.tokens());
        let program = parser.parse()?;
        println!("{:?}", program);
        Ok(Lox {
            program,
            environment: Environment::new(),
        })
    }

    pub fn eval(&mut self) -> Result<()> {
        self.program.eval(&mut self.environment);
        Ok(())
    }
}
