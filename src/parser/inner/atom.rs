use std::fmt;
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

//
//  tokens                              ast
//  10 t2 t3 ...     == parse_atom ==>  10
//
//  tokens           == parse_atom ==>  ast
//  ( t1 t2 ..    )                     expr
//    ~~~~~~~~~~~~
//     parse_expr3
//

/// アトムの構文解析を行います
///
/// 数値、または (EXPR3) がアトムと解釈されて、ASTとして返却されます
/// 引数に渡された tokens はアトムと解釈された次以降のトークン列になります
///
/// 空のトークン列が渡された場合はエラー、開き括弧が閉じられていない場合はエラーとなります。
/// (EXPR3)と EXPR3 を解釈しようとして失敗した場合のエラーもエラーとして伝搬されます
///
///
pub fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token> + fmt::Debug,
{
    log::debug!("parse_atom {tokens:?}");

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
