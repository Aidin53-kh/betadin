use super::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(String, Expression),
    ExpressionStatement(Expression),
    AssignmentStatement(String, Expression),
    ImportStatement(Vec<String>),
}
