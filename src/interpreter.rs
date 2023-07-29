use std::ops::Deref;

use crate::annotations::WithAnnot;
use crate::ast::AstKind;
use crate::errors::InterpreterError;
use crate::errors::InterpreterErrorKind;
use crate::locations::Loc;
use crate::operations::BinaryOperationKind;
use crate::operations::UnaryOperationKind;
use crate::Ast;

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    #[allow(clippy::only_used_in_recursion)]
    pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
        let Loc(from, to) = expr.location();
        match expr.deref() {
            AstKind::Number(ref n) => {
                i64::try_from(*n).map_err(|_| InterpreterErrorKind::Overflow.with(from, to))
            }
            AstKind::UnaryOperation {
                ref operation,
                ref expression,
            } => {
                let value = self.eval(expression)?;
                match operation.deref() {
                    UnaryOperationKind::Plus => Ok(value),
                    UnaryOperationKind::Minus => Ok(-value),
                }
            }
            AstKind::BinaryOperation {
                ref operation,
                ref left_hand_side,
                ref right_hand_side,
            } => {
                let lhs = self.eval(left_hand_side)?;
                let rhs = self.eval(right_hand_side)?;
                match operation.deref() {
                    BinaryOperationKind::Addition => Ok(lhs + rhs),
                    BinaryOperationKind::Subtract => Ok(lhs - rhs),
                    BinaryOperationKind::Multiply => Ok(lhs * rhs),
                    BinaryOperationKind::Division => {
                        if rhs == 0 {
                            Err(InterpreterErrorKind::ZeroDivision.with(from, to))
                        } else {
                            Ok(lhs / rhs)
                        }
                    }
                }
            }
        }
    }
}
