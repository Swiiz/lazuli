use crate::parser::Token;

pub fn build_ast(tokens: Vec<Token>) -> Option<Program> {
    Program::build(&tokens)
}

pub trait ASTNode: Sized {
    fn build(tokens: &[Token]) -> Option<Self>;
}

pub struct Program {}

impl ASTNode for Program {
    fn build(tokens: &[Token]) -> Option<Self> {
        Some(Self {})
    }
}
