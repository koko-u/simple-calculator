use std::fmt;
use std::iter::Peekable;
use std::ops::Deref;

use crate::annotations::WithAnnot;
use crate::ast::Ast;
use crate::ast::AstKind;
use crate::errors::ParseError;
use crate::locations::merge;
use crate::locations::Loc;
use crate::operations::BinaryOperationKind;
use crate::tokens::Token;
use crate::tokens::TokenKind;

use super::expr2::parse_expr2;

//
// tokens                              ast
// t1 t2 t3 t4 ...  == parse_expr3 ==>   a1
//                                     ／  ＼
//                                   a2     a3
//                                 ／  ＼
//                               a4     a5
//
//
// トークン列を解析して、先頭の項を EXPR3 として AST に解釈する
// 先頭の項が EXPR3 でなければエラーとなる

/// EXPR3 を構文解析します
///
/// EXPR2 - EXPR2 + EXPR2 + EXPR2 + EXPR2 のように、同じ結合優先順の +, - 二項演算子で EXPR2 が
/// 続いているとしてトークンを解釈します
///
pub fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token> + fmt::Debug,
{
    log::debug!("parse_expr3 {tokens:?}");

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
