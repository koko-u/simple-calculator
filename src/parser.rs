use crate::ast::Ast;
use crate::parser::parse_errors::ParseError;
use crate::tokens::Token;

use self::inner::expr::parse_expr;

pub mod ops;
pub mod parse_errors;

mod inner;

//
// tokens                            ast
// t1 t2 t3 t4 ...  == parse_expr==>   a1
//                                   ／  ＼
//                                 a2     a3
//                               ／  ＼
//                             a4     a5
//
// rest tokens
//              ??
//
pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ast = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(token) => Err(ParseError::RedundantExpression(token)),
        None => Ok(ast),
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::AstKind;
    use crate::parser::ops::BinaryOperationKind;
    use crate::parser::ops::UnaryOperationKind;
    use crate::tokens::annotations::WithAnnot;
    use crate::tokens::TokenKind::*;
    use pretty_assertions::assert_eq;

    use super::parse;

    #[test]
    fn test_parse() {
        // 1 + 2 * 3 - -10
        let ast = parse(vec![
            Number(1).with(0, 1),
            Plus.with(2, 3),
            Number(2).with(4, 5),
            Asterisk.with(6, 7),
            Number(3).with(8, 9),
            Minus.with(10, 11),
            Minus.with(12, 13),
            Number(10).with(13, 15),
        ]);

        //
        //          -
        //         /  \
        //        +     -
        //      /   \     \
        //     1      *    10
        //          /   \
        //         2     3
        //
        let expr1 = AstKind::BinaryOperation {
            operation: BinaryOperationKind::Multiply.with(6, 7),
            left_hand_side: AstKind::Number(2).with(4, 5).into(),
            right_hand_side: AstKind::Number(3).with(8, 9).into(),
        }
        .with(4, 9);
        let expr2 = AstKind::BinaryOperation {
            operation: BinaryOperationKind::Addition.with(2, 3),
            left_hand_side: AstKind::Number(1).with(0, 1).into(),
            right_hand_side: expr1.into(),
        }
        .with(0, 9);
        let expr3 = AstKind::UnaryOperation {
            operation: UnaryOperationKind::Minus.with(12, 13),
            expression: AstKind::Number(10).with(13, 15).into(),
        }
        .with(12, 15);
        let expr4 = AstKind::BinaryOperation {
            operation: BinaryOperationKind::Subtract.with(10, 11),
            left_hand_side: expr2.into(),
            right_hand_side: expr3.into(),
        }
        .with(0, 15);
        assert_eq!(ast, Ok(expr4));
    }
}
