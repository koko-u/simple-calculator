use crate::parser::ops::BinaryOperation;
use crate::parser::ops::UnaryOperation;
use crate::tokens::annotations::Annot;
use crate::tokens::annotations::WithAnnot;

/// abstract syntax tree kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstKind {
    /// number
    Number(u64),
    /// unary operation kind
    UnaryOperation {
        operation: UnaryOperation,
        expression: Box<Ast>,
    },
    /// binary operation kind
    BinaryOperation {
        operation: BinaryOperation,
        left_hand_side: Box<Ast>,
        right_hand_side: Box<Ast>,
    },
}

/// abstract syntax tree
pub type Ast = Annot<AstKind>;

impl WithAnnot for AstKind {}
