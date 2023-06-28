use std::iter::Peekable;
use std::ops::Deref;

use crate::ast::AstKind;
use crate::parser::inner::expr3::parse_expr3;
use crate::parser::parse_errors::ParseError;
use crate::parser::Ast;
use crate::tokens::annotations::WithAnnot;
use crate::tokens::locations::Loc;
use crate::tokens::Token;
use crate::tokens::TokenKind;

pub fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(token) => match token.deref() {
            TokenKind::Number(n) => {
                let Loc(s, e) = token.location();
                Ok(AstKind::Number(*n).with(s, e))
            }
            TokenKind::LParen => {
                let expr3 = parse_expr3(tokens)?;
                match tokens.next() {
                    Some(t) => {
                        if let &TokenKind::RParen = t.deref() {
                            Ok(expr3)
                        } else {
                            Err(ParseError::RedundantExpression(t))
                        }
                    }
                    None => Err(ParseError::UnclosedParen(token)),
                }
            }
            _ => Err(ParseError::InvalidExpression(token)),
        },
        None => Err(ParseError::Eof),
    }
}
