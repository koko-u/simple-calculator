use crate::annotations::Annot;
use crate::annotations::WithAnnot;

/// The token kind resulting from lexical analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum TokenKind {
    /// positive number
    #[display(fmt = "{}", _0)]
    Number(u64),
    /// plus sign +
    #[display(fmt = "+")]
    Plus,
    /// minus sign -
    #[display(fmt = "-")]
    Minus,
    /// multiply sign *
    #[display(fmt = "*")]
    Asterisk,
    /// division sign /
    #[display(fmt = "/")]
    Slash,
    /// open paren (
    #[display(fmt = "(")]
    LParen,
    /// close paren )
    #[display(fmt = ")")]
    RParen,
}

/// The token type
///
/// token kind and it's location
pub type Token = Annot<TokenKind>;

impl WithAnnot for TokenKind {}
