use crate::annotations::Annot;
use crate::annotations::WithAnnot;

/// unary operation kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperationKind {
    /// plus sign
    Plus,
    /// minus sign
    Minus,
}

/// unary operation type
pub type UnaryOperation = Annot<UnaryOperationKind>;

impl WithAnnot for UnaryOperationKind {}

/// binary operation kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperationKind {
    /// addition
    Addition,
    /// subtraction
    Subtract,
    /// multiplication
    Multiply,
    /// division
    Division,
}

/// binary operation type
pub type BinaryOperation = Annot<BinaryOperationKind>;

impl WithAnnot for BinaryOperationKind {}
