use crate::{Had, TBoard};

pub enum KifPatArea {
    All(TBoard),
}

#[derive(Debug)]
pub struct KifSearchPattern {
    pub(self) rect: Option<(TBoard, bool)>,
    pub(self) had: Option<[Had; 2]>,
}
impl KifSearchPattern {
    pub const fn new() -> Self {
        Self {
            rect: None,
            had: None,
        }
    }
}

pub type KifPat = KifSearchPattern;

pub fn search(pat: KifPat) {
    todo!("一旦\npat: {:#?}", pat)
}
