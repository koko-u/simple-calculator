use std::iter::Peekable;
use std::ops::Deref;

use crate::ast::Ast;
use crate::ast::AstKind;
use crate::parser::ops::UnaryOperationKind;
use crate::parser::parse_errors::ParseError;
use crate::tokens::annotations::WithAnnot;
use crate::tokens::locations::merge;
use crate::tokens::locations::Loc;
use crate::tokens::Token;
use crate::tokens::TokenKind;

use super::atom::parse_atom;

pub fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
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
