use std::str;

use crate::lexer::inner::lex_number;
use crate::lexer::lex_errors::LexError;
use crate::lexer::lex_errors::LexErrorKind;
use crate::tokens::annotations::WithAnnot;
use crate::tokens::Token;
use crate::tokens::TokenKind;

mod inner;
pub mod lex_errors;

/// lexer
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    use crate::lexer::LexErrorKind::*;
    use crate::lexer::TokenKind::*;

    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    while pos < input.len() {
        if input[pos].is_ascii_whitespace() {
            pos += 1;
            continue;
        }

        match input[pos] {
            b'0'..=b'9' => {
                let (token, new_pos) = lex_number(input, pos);
                tokens.push(token);
                pos = new_pos;
            }
            b'+' => {
                tokens.push(Plus.with(pos, pos + 1));
                pos += 1
            }
            b'-' => {
                tokens.push(Minus.with(pos, pos + 1));
                pos += 1
            }
            b'*' => {
                tokens.push(Asterisk.with(pos, pos + 1));
                pos += 1
            }
            b'/' => {
                tokens.push(Slash.with(pos, pos + 1));
                pos += 1
            }
            b'(' => {
                tokens.push(LParen.with(pos, pos + 1));
                pos += 1
            }
            b')' => {
                tokens.push(RParen.with(pos, pos + 1));
                pos += 1
            }
            otherwise => {
                return Err(InvalidChar(otherwise as char).with(pos, pos + 1));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    use crate::lexer::lex_errors::LexErrorKind;
    use crate::tokens::annotations::WithAnnot;
    use crate::tokens::TokenKind::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn lex_test() {
        let lexed = lex("1 + 2 * 3 - -10");
        assert_eq!(
            lexed,
            Ok(vec![
                Number(1).with(0, 1),
                Plus.with(2, 3),
                Number(2).with(4, 5),
                Asterisk.with(6, 7),
                Number(3).with(8, 9),
                Minus.with(10, 11),
                Minus.with(12, 13),
                Number(10).with(13, 15)
            ])
        )
    }

    #[test]
    fn lex_err() {
        let lexed = lex("1 @ 4");
        assert_eq!(lexed, Err(LexErrorKind::InvalidChar('@').with(2, 3)));
    }

    #[test]
    fn lex_empty() {
        let lexed = lex("");
        assert_eq!(lexed, Ok(vec![]));
    }
}
