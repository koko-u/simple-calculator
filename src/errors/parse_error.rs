use std::error;

use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum ParseError {
    /// encounter unexpected token
    #[display(fmt = "Unexpected Token {}", _0)]
    UnexpectedToken(Token),
    /// encounter invalid token where the expression is expected.
    #[display(fmt = "Invalid Expresssion {}", _0)]
    InvalidExpression(Token),
    /// encounter invalid token where the operator is expected
    #[display(fmt = "Invalid Operation {}", _0)]
    InvalidOperator(Token),
    /// Opening brackets not closed
    #[display(fmt = "Un-Closed Paren {}", _0)]
    UnclosedParen(Token),
    /// finish parsing , but it has an extra token.
    #[display(fmt = "Redundant Expression {}", _0)]
    RedundantExpression(Token),
    /// ran out of input during parsing.
    #[display(fmt = "End of file")]
    Eof,
}

impl error::Error for ParseError {}
