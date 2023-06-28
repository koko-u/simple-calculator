use std::iter::Peekable;
use std::ops::Deref;

use crate::ast::Ast;
use crate::ast::AstKind;
use crate::parser::ops::BinaryOperationKind;
use crate::parser::parse_errors::ParseError;
use crate::tokens::annotations::WithAnnot;
use crate::tokens::locations::merge;
use crate::tokens::locations::Loc;
use crate::tokens::Token;
use crate::tokens::TokenKind;

use super::expr1::parse_expr1;

pub fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut expr = parse_expr1(tokens)?;
    loop {
        match tokens.peek().map(Deref::deref) {
            Some(&TokenKind::Asterisk) => {
                let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
                let op = BinaryOperationKind::Multiply.with(operation_start, operation_end);
                let expr1 = parse_expr1(tokens)?;
                let Loc(expr_start, expr_end) = merge(expr.location(), expr1.location());
                let ast = AstKind::BinaryOperation {
                    operation: op,
                    left_hand_side: expr.into(),
                    right_hand_side: expr1.into(),
                };

                expr = ast.with(expr_start, expr_end);
            }
            Some(&TokenKind::Slash) => {
                let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
                let op = BinaryOperationKind::Division.with(operation_start, operation_end);
                let expr1 = parse_expr1(tokens)?;
                let Loc(expr_start, expr_end) = merge(expr.location(), expr1.location());
                let ast = AstKind::BinaryOperation {
                    operation: op,
                    left_hand_side: expr.into(),
                    right_hand_side: expr1.into(),
                };

                expr = ast.with(expr_start, expr_end);
            }
            _ => return Ok(expr),
        }
    }
}
