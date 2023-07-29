use crate::locations::Loc;

pub(super) struct PrintWith<'a> {
    input: &'a str,
}
impl<'a> PrintWith<'a> {
    pub(super) fn with(&self, Loc(from, to): Loc) {
        eprintln!("{}", self.input);
        eprintln!("{}{}", " ".repeat(from), "^".repeat(to - from));
    }
}

pub(super) fn print_error(input: &str) -> PrintWith {
    PrintWith { input }
}
