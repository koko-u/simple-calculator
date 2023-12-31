use std::fmt;
use std::iter::Peekable;
use std::ops::Deref;

use crate::annotations::WithAnnot;
use crate::ast::Ast;
use crate::ast::AstKind;
use crate::errors::ParseError;
use crate::locations::merge;
use crate::locations::Loc;
use crate::operations::UnaryOperationKind;
use crate::tokens::Token;
use crate::tokens::TokenKind;

use super::atom::parse_atom;

//
// tokens                              ast
// + t2 t3 t4 ...  == parse_expr1 ==>   +
//   ~~~~~~~~                           |
//   parse_atom                        atom
//
// tokens                              ast
// - t2 t3 t4 ...  == parse_expr1 ==>   -
//   ~~~~~~~~                           |
//   parse_atom                        atom
//
//
// tokens                              ast
// t2 t3 t4 ...  == parse_expr1 ==>   atom
// ~~~~~~~~
// parse_atom
//
//

/// EXPR1 を構文解析します
///
/// tokens の先頭に + または - が現われた場合に、次を ATOM として解析します
/// アトムの構文解析エラーが単純に伝搬します
pub fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token> + fmt::Debug,
{
    log::debug!("parse_expr1 {tokens:?}");

    match tokens.peek().map(Deref::deref) {
        Some(&TokenKind::Plus) => {
            let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
            let op = UnaryOperationKind::Plus.with(operation_start, operation_end);
            let atom = parse_atom(tokens)?;
            let Loc(expr_start, expr_end) = merge(op.location(), atom.location());
            let ast = AstKind::UnaryOperation {
                operation: op,
                expression: atom.into(),
            };
            Ok(ast.with(expr_start, expr_end))
        }
        Some(&TokenKind::Minus) => {
            let Loc(operation_start, operation_end) = tokens.next().unwrap().location();
            let op = UnaryOperationKind::Minus.with(operation_start, operation_end);
            let atom = parse_atom(tokens)?;
            let Loc(expr_start, expr_end) = merge(op.location(), atom.location());
            let ast = AstKind::UnaryOperation {
                operation: op,
                expression: atom.into(),
            };
            Ok(ast.with(expr_start, expr_end))
        }
        _ => parse_atom(tokens),
    }
}
