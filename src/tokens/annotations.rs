use super::locations::Loc;

/// 型Tの値に対してその位置情報を付与した構造体です
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Display,
)]
#[display(fmt = "{}: {}", loc, value)]
pub struct Annot<T> {
    #[deref]
    #[deref_mut]
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
    pub fn location(&self) -> Loc {
        self.loc
    }
}

pub trait WithAnnot: Sized {
    fn with(self, start_pos: usize, end_pos: usize) -> Annot<Self> {
        Annot::new(self, Loc(start_pos, end_pos))
    }
}
