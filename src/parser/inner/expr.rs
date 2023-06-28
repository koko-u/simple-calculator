use std::iter::Peekable;

use crate::ast::Ast;
use crate::parser::parse_errors::ParseError;
use crate::tokens::Token;

use super::expr3::parse_expr3;

pub fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}
