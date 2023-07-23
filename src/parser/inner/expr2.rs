use std::fmt;
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

//
// tokens                              ast
// t1 t2 t3 t4 ...  == parse_expr2 ==>   a1
//                                     ／  ＼
//                                   a2     a3
//                                 ／  ＼
//                               a4     a5
//
// 1. まずは tokens を先頭から EXPR1 だと考えて解析する、その結果として得られた AST を expr に保持する
// 2. tokens は parse_expr1 によって消費されて、残りのシーケンスになる
//
//     tokens                              ast
//     t1 t2 t3 t4 ...  == parse_expr2 ==>   a1
//     ~~~~~~
//     parse_expr1 => expr
//
//     tokens
//           t3 t4 ...
//
//
// 3. 残った tokens の先頭を満て、 * または / の場合には、これが EXPR2_Loop = ("*" | "-") EXPR1 EXPR2_Loop | ε
// 4. expr * expr1 を expr に代入して、次の解析を続ける
//
//     tokens                              ast
//     t1 t2 t3 t4 ...  == parse_expr2 ==>
//     ~~~~~~
//     parse_expr1 => expr      .....>   expr = expr * expr1
//
//     tokens
//           * t4 t5 ...
//             ~~~~~~
//             parse_expr1 => expr1
//
//     tokens
//                     t6 t7 ...
//
//
// 5. かけ算や割り算の計算でなくなったらループを抜ける
//
//     tokens                              ast
//     t1 t2 t3 t4 ...  == parse_expr2 ==>
//                                              *
//                                            ／  ＼
//                                           *     expr1
//                                         ／  ＼
//                                      expr   expr1
//
//     tokens
//                 t6 t7 ...
//
//

/// EXPR2 を構文解析します
///
/// EXPR1 * EXPR1 / EXPR1 / EXPR1 * EXPR1 のように、同じ結合優先順の *, / 二項演算子で EXPR1 が
/// 続いているとしてトークンを解釈します
///
/// EXPR1 として解釈しようとして失敗したエラーが伝搬します
///
pub fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token> + fmt::Debug,
{
    log::debug!("parse_expr2 {tokens:?}");

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
