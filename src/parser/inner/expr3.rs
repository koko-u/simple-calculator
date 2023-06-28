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

use super::expr2::parse_expr2;

// トークン列を解析して、先頭の項を EXPR3 として AST に解釈する
// 先頭の項が EXPR3 でなければエラーとなる
pub fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut expr = parse_expr2(tokens)?;
    loop {
        match tokens.peek().map(Deref::deref) {
            Some(&TokenKind::Plus) => {
                let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
                let op = BinaryOperationKind::Addition.with(operation_start, operation_end);
                let expr2 = parse_expr2(tokens)?;
                let Loc(expr_start, expr_end) = merge(expr.location(), expr2.location());
                let ast = AstKind::BinaryOperation {
                    operation: op,
                    left_hand_side: expr.into(),
                    right_hand_side: expr2.into(),
                };

                expr = ast.with(expr_start, expr_end);
            }
            Some(&TokenKind::Minus) => {
                let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
                let op = BinaryOperationKind::Subtract.with(operation_start, operation_end);
                let expr2 = parse_expr2(tokens)?;
                let Loc(expr_start, expr_end) = merge(expr.location(), expr2.location());
                let ast = AstKind::BinaryOperation {
                    operation: op,
                    left_hand_side: expr.into(),
                    right_hand_side: expr2.into(),
                };

                expr = ast.with(expr_start, expr_end);
            }
            _ => return Ok(expr),
        }
    }
}
