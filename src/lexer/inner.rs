use std::str;

use crate::tokens::Token;
use crate::tokens::TokenKind;
use crate::tokens::annotations::WithAnnot;

pub fn lex_number(input: &[u8], pos: usize) -> (Token, usize) {
    let start_pos = pos;

    let new_pos = consume_bytes(input, pos, u8::is_ascii_digit);

    let n = str::from_utf8(&input[start_pos..new_pos]).unwrap();
    let n = n.parse::<u64>().unwrap();
    let token = TokenKind::Number(n).with(start_pos, new_pos);

    (token, new_pos)
}

fn consume_bytes(input: &[u8], pos: usize, condition: impl Fn(&u8) -> bool) -> usize {
    let mut current_pos = pos;

    while current_pos < input.len() {
        if condition(&input[current_pos]) {
            current_pos += 1;
        } else {
            break;
        }
    }

    current_pos
}
