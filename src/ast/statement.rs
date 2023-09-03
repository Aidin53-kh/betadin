use super::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
    AssignmentStatement(String, Expression)
}
