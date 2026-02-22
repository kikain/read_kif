use crate::{Board, Had, Piece, TBoard};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KifPatPiece {
    Ignore,
    Handle(Piece),
}
impl PartialEq<Piece> for KifPatPiece {
    fn eq(&self, other: &Piece) -> bool {
        match self {
            Self::Ignore => true,
            Self::Handle(piece) => *piece == *other,
        }
    }
}
pub struct Rect {
    start: (u32, u32),
    range: (u32, u32),
}
impl Rect {
    pub const fn new() -> Self {
        Self {
            start: (0, 0),
            range: (0, 0),
        }
    }
    pub const fn with_end(end: (u32, u32)) -> Self {
        Self {
            start: (0, 0),
            range: end,
        }
    }
    pub const fn with_start_end(start: (u32, u32), end: (u32, u32)) -> Self {
        Self {
            start,
            range: (end.0 - start.0, end.1 - start.1),
        }
    }
    pub const fn is_inside(&self, x: u32, y: u32) -> bool {
        if self.is_inside_x(x) && self.is_inside_y(y) {
            true
        } else {
            false
        }
    }
    pub const fn is_inside_x(&self, val: u32) -> bool {
        if self.start.0 <= val && val <= self.start.0 + self.range.0 {
            return true;
        }
        false
    }
    pub const fn is_inside_y(&self, val: u32) -> bool {
        if self.start.1 <= val && val <= self.start.1 + self.range.1 {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KifPatBoard {
    board: [[KifPatPiece; 9]; 9],
    some_start: (usize, usize),
}
impl KifPatBoard {
    pub const fn new() -> Self {
        Self {
            board: [[KifPatPiece::Ignore; 9]; 9],
            some_start: (9, 9), // never match
        }
    }
    pub const fn with_board(board: TBoard) -> Self {
        let mut temp = [[KifPatPiece::Ignore; 9]; 9];
        let mut ix: usize = 0;
        while ix <= 8 {
            let mut iy: usize = 0;
            while iy <= 8 {
                temp[ix][iy] = KifPatPiece::Handle(board[ix][iy]);
                iy += 1;
            }
            ix += 1;
        }
        Self {
            board: temp,
            some_start: (0, 0),
        }
    }
    pub const fn with_board_area(board: TBoard, area: Rect) -> Self {
        let mut temp = [[KifPatPiece::Ignore; 9]; 9];
        let mut ix: usize = 0;
        let mut some_start: (usize, usize) = (9, 9);
        while ix <= 8 {
            if !area.is_inside_x(ix as u32) {
                temp[ix] = [KifPatPiece::Ignore; 9];
                ix += 1;
                continue;
            } else if some_start.0 == 9 {
                some_start.0 = ix;
            }
            let mut iy: usize = 0;
            while iy <= 8 {
                if !area.is_inside_y(iy as u32) {
                    temp[ix][iy] = KifPatPiece::Ignore;
                    iy += 1;
                    continue;
                } else if some_start.1 == 9 {
                    some_start.1 = iy;
                }
                temp[ix][iy] = KifPatPiece::Handle(board[ix][iy]);
                iy += 1;
            }
            ix += 1;
        }
        Self {
            board: temp,
            some_start,
        }
    }
    pub const fn is_all_ignored(&self) -> bool {
        let mut ix: usize = 0;
        while ix <= 8 {
            let mut iy: usize = 0;
            while iy <= 8 {
                if let KifPatPiece::Ignore = self.board[ix][iy] {
                } else {
                    return false;
                }
                iy += 1
            }
            ix += 1;
        }
        true
    }
    pub fn search_all(&self, target: TBoard) -> bool {
        let mut ix: usize = 0;
        while ix <= 8 {
            let mut iy: usize = 0;
            while iy <= 8 {
                if self.board[ix][iy] != target[ix][iy] {
                    return false;
                }
                iy += 1;
            }
            ix += 1;
        }
        true
    }
    pub fn search(&self, target: TBoard) -> bool {
        if self.some_start == (9, 9) {
            return true;
        }
        if self.board[self.some_start.0][self.some_start.1]
            != target[self.some_start.0][self.some_start.1]
        {
            return false;
        }
        self.search_all(target)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KifPatHad {
    king: Option<u32>,
    gold: Option<u32>,
    silver: Option<u32>,
    knight: Option<u32>,
    spear: Option<u32>,
    rook: Option<u32>,
    bishop: Option<u32>,
    pawn: Option<u32>,
}
impl KifPatHad {
    pub const fn new() -> Self {
        Self {
            king: None,
            gold: None,
            silver: None,
            knight: None,
            spear: None,
            rook: None,
            bishop: None,
            pawn: None,
        }
    }
    pub const fn is_all_ignored(&self) -> bool {
        if self.king.is_none()
            && self.gold.is_none()
            && self.silver.is_none()
            && self.knight.is_none()
            && self.spear.is_none()
            && self.rook.is_none()
            && self.bishop.is_none()
            && self.pawn.is_none()
        {
            true
        } else {
            false
        }
    }
    pub(self) const fn to_option(self) -> Option<KifPatHad> {
        if self.is_all_ignored() {
            None
        } else {
            Some(self)
        }
    }
    const fn had_eq(me: Option<u32>, other: u32) -> bool {
        match me {
            None => true,
            Some(val) => val == other,
        }
    }
    pub const fn search(&self, target: Had) -> bool {
        if self.is_all_ignored() {
            true
        } else {
            Self::had_eq(self.king, target.king)
                && Self::had_eq(self.gold, target.gold)
                && Self::had_eq(self.silver, target.silver)
                && Self::had_eq(self.knight, target.knight)
                && Self::had_eq(self.spear, target.spear)
                && Self::had_eq(self.rook, target.rook)
                && Self::had_eq(self.bishop, target.bishop)
                && Self::had_eq(self.pawn, target.pawn)
        }
    }
}
impl Default for KifPatHad {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Had> for KifPatHad {
    fn from(had: Had) -> Self {
        Self {
            king: Some(had.king),
            gold: Some(had.gold),
            silver: Some(had.silver),
            knight: Some(had.knight),
            spear: Some(had.spear),
            rook: Some(had.rook),
            bishop: Some(had.bishop),
            pawn: Some(had.pawn),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KifSearchPattern {
    pub(self) board: Option<KifPatBoard>,
    pub(self) had_up: Option<KifPatHad>,
    pub(self) had_down: Option<KifPatHad>,
}
impl KifSearchPattern {
    const DEFAULT: Self = Self {
        board: None,
        had_up: None,
        had_down: None,
    };
    pub const fn new() -> Self {
        Self::DEFAULT
    }
    pub const fn with_board(board: KifPatBoard) -> Self {
        Self {
            board: if board.is_all_ignored() {
                None
            } else {
                Some(board)
            },
            ..Self::DEFAULT
        }
    }
    pub const fn with_had(had_up: KifPatHad, had_down: KifPatHad) -> Self {
        Self {
            had_up: had_up.to_option(),
            had_down: had_down.to_option(),
            ..Self::DEFAULT
        }
    }
    pub fn search(&self, target: &Board) -> bool {
        if let Some(board_pat) = self.board {
            if !board_pat.search(target.board) {
                return false;
            }
        }
        if let Some(had_up_pat) = self.had_up {
            if !had_up_pat.search(target.had_up) {
                return false;
            }
        }
        if let Some(had_down_pat) = self.had_down {
            if !had_down_pat.search(target.had_down) {
                return false;
            }
        }
        true
    }
}

pub type KifPat = KifSearchPattern;
