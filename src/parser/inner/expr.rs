use std::fmt;
use std::iter::Peekable;

use crate::ast::Ast;
use crate::errors::ParseError;
use crate::tokens::Token;

use super::expr3::parse_expr3;

//
// tokens                            ast
// t1 t2 t3 t4 ...  == parse_expr==>   a1
//                                   ／  ＼
//                                 a2     a3
//                               ／  ＼
//                             a4     a5
//
pub fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token> + fmt::Debug,
{
    parse_expr3(tokens)
}
