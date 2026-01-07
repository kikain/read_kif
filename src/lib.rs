mod reader;
pub mod searcher;

#[derive(Clone, Copy, Debug, Default)]
pub enum PieceEnum {
    #[default]
    Empty,
    King,
    Gold,
    Silver,
    Knight,
    Spear,
    Rook,
    Bishop,
    Pawn,
}
impl PieceEnum {
    pub(crate) fn _get_piece_heads() -> [char; 15] {
        [
            '玉', '金', '銀', '桂', '香', '飛', '角', '歩', '成', '全', '圭', '杏', 'と', '龍',
            '竜',
        ]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Piece {
    pub piece: PieceEnum,
    pub is_down: bool,
    pub is_promoted: bool,
}
impl Piece {
    const DEFAULT: Self = Self {
        piece: PieceEnum::Empty,
        is_down: false,
        is_promoted: false,
    };
    ///new func for initializing the Board
    pub(self) const fn new_b(pieces: [PieceEnum; 9], is_down: bool) -> [Self; 9] {
        let mut temp = [Self::new(PieceEnum::Empty, is_down, false); 9];
        let mut i: usize = 1;
        while i < 9 {
            temp[i] = Self::new(pieces[i], is_down, false);
            i += 1;
        }
        temp
    }
    pub const fn new(piece: PieceEnum, is_down: bool, is_promoted: bool) -> Self {
        Piece {
            piece,
            is_down,
            is_promoted,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        if x > 8 {
            panic!("x ranges from 0 to 8, but received {}", x);
        }
        if y > 8 {
            panic!("y ranges from 0 to 8, but received {}", y);
        }
        Pos { x, y }
    }
    ///self to MovePos
    pub(crate) const fn to_mp(&self) -> MovePos {
        MovePos::Board(*self)
    }
}

#[derive(Debug, Clone, Copy)]
enum MovePos {
    Board(Pos),
    Had(Piece),
}

#[derive(Debug)]
pub struct Move {
    pub(crate) from: MovePos,
    pub to: Pos,
    pub do_promotion: bool,
}
impl Move {}

pub enum PM {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy)]
pub struct Had {
    pub king: u32,
    pub gold: u32,
    pub silver: u32,
    pub knight: u32,
    pub spear: u32,
    pub rook: u32,
    pub bishop: u32,
    pub pawn: u32,
}
impl Default for Had {
    fn default() -> Self {
        Had {
            king: 0,
            gold: 0,
            silver: 0,
            knight: 0,
            spear: 0,
            rook: 0,
            bishop: 0,
            pawn: 0,
        }
    }
}
impl Had {
    const DEFAULT: Self = Had {
        king: 0,
        gold: 0,
        silver: 0,
        knight: 0,
        spear: 0,
        rook: 0,
        bishop: 0,
        pawn: 0,
    };
    pub fn get(&self, key: PieceEnum) -> u32 {
        use PieceEnum::*;
        match key {
            Empty => panic!("had in not 'Empty'"),
            King => self.king,
            Gold => self.gold,
            Silver => self.silver,
            Knight => self.knight,
            Spear => self.spear,
            Rook => self.rook,
            Bishop => self.bishop,
            Pawn => self.pawn,
        }
    }
    fn adjust_count(field: &mut u32, val: i32) {
        let tmp = *field as i64 + val as i64;
        if tmp < 0 {
            panic!("attempt to set negative had count: {}", tmp);
        }
        if tmp > u32::MAX as i64 {
            panic!("attempt to overflow had count: {}", tmp);
        }
        *field = tmp as u32;
    }
    // Moves the specified number of frames up or down.
    // You'll probably use it like this:
    // ```rust
    // let has = Had::default();
    // has.inc(PieceEnum::Knight,+1);
    // assert_eq!(has.get(PieceEnum::Knight),1)
    // has.pawn = 2
    // assert_
    // ```
    pub fn inc(&mut self, key: PieceEnum, val: i32) {
        use PieceEnum::*;
        match key {
            Empty => panic!("had in not 'Empty'"),
            King => Self::adjust_count(&mut self.king, val),
            Gold => Self::adjust_count(&mut self.gold, val),
            Silver => Self::adjust_count(&mut self.silver, val),
            Knight => Self::adjust_count(&mut self.knight, val),
            Spear => Self::adjust_count(&mut self.spear, val),
            Rook => Self::adjust_count(&mut self.rook, val),
            Bishop => Self::adjust_count(&mut self.bishop, val),
            Pawn => Self::adjust_count(&mut self.pawn, val),
        }
    }
}

pub type TBoard = [[Piece; 9]; 9];
pub struct Board {
    pub board: TBoard,
    pub has_down: Had,
    pub has_up: Had,
}
impl Default for Board {
    fn default() -> Self {
        Board::empty()
    }
}
impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            board: self.board,
            has_down: self.has_down,
            has_up: self.has_up,
        }
    }
}
impl Board {
    pub fn new() -> Self {
        Self::normal()
    }
    pub fn empty() -> Self {
        Board {
            board: [[Piece::default(); 9]; 9],
            has_down: Had::default(),
            has_up: Had::default(),
        }
    }
    pub const fn normal() -> Self {
        use PieceEnum::*;
        Board {
            board: [
                Piece::new_b(
                    [
                        Spear, Knight, Silver, Gold, King, Gold, Silver, Knight, Spear,
                    ],
                    false,
                ),
                Piece::new_b(
                    [
                        Empty, Rook, Empty, Empty, Empty, Empty, Empty, Bishop, Empty,
                    ],
                    false,
                ),
                [Piece::new(Pawn, false, false); 9],
                [Piece::DEFAULT; 9],
                [Piece::DEFAULT; 9],
                [Piece::DEFAULT; 9],
                [Piece::new(Pawn, true, false); 9],
                Piece::new_b(
                    [
                        Empty, Rook, Empty, Empty, Empty, Empty, Empty, Bishop, Empty,
                    ],
                    true,
                ),
                Piece::new_b(
                    [
                        Spear, Knight, Silver, Gold, King, Gold, Silver, Knight, Spear,
                    ],
                    true,
                ),
            ],
            has_down: Had::DEFAULT,
            has_up: Had::DEFAULT,
        }
    }
    pub fn next(&mut self, m: &Move) -> Self {
        let to_mp = m.to.to_mp();
        let from_mp = m.from;
        let captured = self.get(to_mp).piece;
        let moving = self.get(from_mp);
        self.set(to_mp, moving);
        self.set(from_mp, Piece::default());
        if let PieceEnum::Empty = captured {
        } else {
            if moving.is_down {
                self.has_down.inc(captured, 1);
            } else {
                self.has_up.inc(captured, 1);
            }
        }
        Board {
            board: self.board,
            has_down: self.has_down.clone(),
            has_up: self.has_up.clone(),
        }
    }
    const fn get(&self, pos: MovePos) -> Piece {
        match pos {
            MovePos::Board(xy) => self.board[xy.x][xy.y],
            MovePos::Had(p) => p,
        }
    }
    fn set(&mut self, pos: MovePos, item: Piece) {
        match pos {
            MovePos::Board(xy) => {
                self.board[xy.x][xy.y] = item;
            }
            MovePos::Had(p) => {
                let val = if let PieceEnum::Empty = item.piece {
                    -1
                } else {
                    1
                };
                if p.is_down {
                    self.has_down.inc(p.piece, val);
                } else {
                    self.has_up.inc(p.piece, val);
                }
            }
        }
    }
    // さらに別のプリセットがあれば同様に関数を追加できます（compact(), handicap(), ...）
}
impl From<TBoard> for Board {
    fn from(board: TBoard) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

pub type TKif = Vec<Move>;
pub struct Kif {
    pub kif: TKif,
    pub(crate) map: Board,
}
impl Kif {
    ///from_vec for test
    pub fn t_from_vec(kif: TKif) -> Self {
        Self {
            kif,
            map: Board::normal(),
        }
    }
    pub fn with_board(kif: TKif, board: Board) -> Self {
        Self { kif, map: board }
    }
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_options, moves) = reader::read_kif(path, &reader::Opt::default())?;
        Ok(Self::t_from_vec(moves))
    }
    pub fn search(pat: searcher::KifPat) {
        searcher::search(pat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;
    type TestRes<E> = Result<(), Box<E>>;

    #[test]
    fn test_read() -> TestRes<dyn std::error::Error> {
        let (_ops, kif) = reader::read_kif(r".\data\kif1.kif2", &reader::Opt::default())?;
        println!("{:#?}", kif);
        Ok(())
    }

    #[test]
    fn had_inc_positive() {
        let mut h = Had::default();
        h.inc(PieceEnum::Knight, 1);
        assert_eq!(h.knight, 1);
    }

    #[test]
    fn had_inc_decrement() {
        let mut h = Had::default();
        h.pawn = 2;
        h.inc(PieceEnum::Pawn, -1);
        assert_eq!(h.pawn, 1);
    }

    #[test]
    #[should_panic]
    fn had_inc_negative_panics() {
        let mut h = Had::default();
        h.inc(PieceEnum::Pawn, -1);
    }

    #[test]
    #[should_panic]
    fn had_inc_overflow_panics() {
        let mut h = Had::default();
        h.pawn = u32::MAX;
        h.inc(PieceEnum::Pawn, 1);
    }
}
