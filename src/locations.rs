/// structure that holds the position of the token in the input characters.
///
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "{}-{}", _0, _1)]
pub struct Loc(
    /// start position
    pub usize,
    /// end position
    pub usize,
);

/// merge two locations
pub fn merge(loc1: Loc, loc2: Loc) -> Loc {
    let start_position = std::cmp::min(loc1.0, loc2.0);
    let end_position = std::cmp::max(loc1.1, loc2.1);
    Loc(start_position, end_position)
}
