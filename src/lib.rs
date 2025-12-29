mod reader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default,Hash)]
pub enum PieceEnum{
    #[default]
    Empty,
    King,
    Gold,
    Silver,
    Knight,
    Spear,
    Hisha,
    Kaku,
    Pawn,
}impl ToString for PieceEnum {
    fn to_string(&self) -> String {
        use PieceEnum::*;
        match self {
            Empty => "Empty".to_string(),
            King => "King".to_string(),
            Gold => "Gold".to_string(),
            Silver => "Silver".to_string(),
            Knight => "Knight".to_string(),
            Spear => "Spear".to_string(),
            Hisha => "Hisha".to_string(),
            Kaku => "Kaku".to_string(),
            Pawn => "Pawn".to_string(),
        }
    }
}impl PieceEnum {
    fn _get_piece_heads() -> [char;9] {
        [
            'E',
            'K',
            'G',
            'S',
            'K',
            'S',
            'H',
            'K',
            'P',
        ]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Piece{
    pub piece:PieceEnum,
    pub is_down:bool,
    pub is_promoted:bool,
}impl Piece {
    ///new func for initializing the Board
    pub fn new_b(piece:PieceEnum,is_down:bool) -> Self {
        Self::new(piece, is_down,false)
    }
    pub fn new(piece:PieceEnum,is_down:bool,is_promoted:bool) -> Self {
        Piece { piece, is_down, is_promoted }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos{
    pub x: usize,
    pub y: usize
}impl Pos {
    pub fn new(x:usize,y:usize) -> Self{
        if x > 8 { panic!("x range is 0~8, but geted is {x}"); }
        if y > 8 { panic!("y range is 0~8, but geted is {y}"); }
        Pos{x,y}
    }
    ///self to MovePos
    pub(crate) fn to_mp(&self) -> MovePos {
        MovePos::Board(*self)
    }
}

#[derive(Debug, Clone, Copy)]
enum MovePos{
    Board(Pos),
    Hased(Piece)
}

#[derive(Debug)]
pub struct Move{
    pub(crate) from: MovePos,
    pub to:Pos,
    pub do_promot:bool
}impl Move {
}

pub enum PM {
    Plus,
    Minus
}

#[derive(Debug,Clone,/*Copy,*/)]
pub struct Hased {
    pub king: u32,
    pub gold: u32,
    pub silver: u32,
    pub knight: u32,
    pub spear: u32,
    pub hisha: u32,
    pub kaku: u32,
    pub pawn: u32,
}impl Default for Hased {
    fn default() -> Self {
        Hased {
            king: 0,
            gold: 0,
            silver: 0,
            knight: 0,
            spear: 0,
            hisha: 0,
            kaku: 0,
            pawn: 0
        }
    }
}impl Hased {
    pub fn get(&self,key:PieceEnum) -> u32 {
        use PieceEnum::*;
        match key {
            Empty => panic!("hased in not 'Empty'"),
            King => self.king,
            Gold => self.gold,
            Silver => self.silver,
            Knight => self.knight,
            Spear => self.spear,
            Hisha => self.hisha,
            Kaku => self.kaku,
            Pawn => self.pawn,
        }
    }
    // Adjust a u32 field by a signed i32 value safely (no wrapping).
    fn adjust_count(field: &mut u32, val: i32) {
        let tmp = *field as i64 + val as i64;
        if tmp < 0 {
            panic!("attempt to set negative hased count: {}", tmp);
        }
        if tmp > u32::MAX as i64 {
            panic!("attempt to overflow hased count: {}", tmp);
        }
        *field = tmp as u32;
    }
    // Moves the specified number of frames up or down.
    // You'll probably use it like this:
    // ```rust
    // let has = Hased::default();
    // has.inc(PieceEnum::Knight,+1);
    // assert_eq!(has.get(PieceEnum::Knight),1)
    // has.pawn = 2
    // assert_
    // ```
    pub fn inc(&mut self,key:PieceEnum,val:i32){
        use PieceEnum::*;
        match key {
            Empty => panic!("hased in not 'Empty'"),
            King => Self::adjust_count(&mut self.king, val),
            Gold => Self::adjust_count(&mut self.gold, val),
            Silver => Self::adjust_count(&mut self.silver, val),
            Knight => Self::adjust_count(&mut self.knight, val),
            Spear => Self::adjust_count(&mut self.spear, val),
            Hisha => Self::adjust_count(&mut self.hisha, val),
            Kaku => Self::adjust_count(&mut self.kaku, val),
            Pawn => Self::adjust_count(&mut self.pawn, val),
        }
    }
}

pub type TBoard = [[Piece;9];9];
pub struct Board{
    pub board:TBoard,
    pub has_down:Hased,
    pub has_up:Hased,
}impl Default for Board {
    fn default() -> Self {
        Board::empty()
    }
}impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            board: self.board.clone(),
            has_down: self.has_down.clone(),
            has_up: self.has_up.clone()
        }
    }
}impl Board {
    pub fn new() -> Self {
        Self::normal()
    }
    pub fn empty() -> Self {
        Board {
            board: [[Piece::default(); 9]; 9],
            has_down: Hased::default(),
            has_up: Hased::default()
        }
    }
    pub fn normal() -> Self {
        use PieceEnum::*;
        Board {
            board: [
                [
                    Piece::new_b(Spear,false),
                    Piece::new_b(Knight,false),
                    Piece::new_b(Silver,false),
                    Piece::new_b(Gold,false),
                    Piece::new_b(King,false),
                    Piece::new_b(Gold,false),
                    Piece::new_b(Silver,false),
                    Piece::new_b(Knight,false),
                    Piece::new_b(Spear,false),
                ],
                [
                    Piece::new_b(Empty,false),
                    Piece::new_b(Hisha,false),
                    Piece::new_b(Empty,false),
                    Piece::new_b(Empty,false),
                    Piece::new_b(Empty,false),
                    Piece::new_b(Empty,false),
                    Piece::new_b(Empty,false),
                    Piece::new_b(Kaku,false),
                    Piece::new_b(Empty,false),
                ],
                [
                    Piece::new_b(Pawn,false); 9
                ],
                [
                    Piece::default(); 9
                ],
                [
                    Piece::default(); 9
                ],
                [
                    Piece::default(); 9
                ],
                [
                    Piece::new_b(Pawn,true); 9
                ],
                [
                    Piece::new_b(Empty,true),
                    Piece::new_b(Hisha,true),
                    Piece::new_b(Empty,true),
                    Piece::new_b(Empty,true),
                    Piece::new_b(Empty,true),
                    Piece::new_b(Empty,true),
                    Piece::new_b(Empty,true),
                    Piece::new_b(Kaku,true),
                    Piece::new_b(Empty,true),
                ],
                [
                    Piece::new_b(Spear,true),
                    Piece::new_b(Knight,true),
                    Piece::new_b(Silver,true),
                    Piece::new_b(Gold,true),
                    Piece::new_b(King,true),
                    Piece::new_b(Gold,true),
                    Piece::new_b(Silver,true),
                    Piece::new_b(Knight,true),
                    Piece::new_b(Spear,true),
                ],
            ],
            has_down: Hased::default(),
            has_up: Hased::default(),
        }
    }
    pub fn next(&mut self,m:&Move) -> Self {
        // 宛先／出発の MovePos（MovePos は Copy して使える）
        let to_mp = m.to.to_mp();
        let from_mp = m.from;

        // 宛先の駒（捕獲対象）を取得
        let captured = self.get(to_mp).piece;
        // 移動する駒を取得して移動する
        let moving = self.get(from_mp);
        self.set(to_mp, moving);
        self.set(from_mp, Piece::default());

        // 捕獲があれば持ち駒に加算
        if captured != PieceEnum::Empty {
            if moving.is_down {
                self.has_up.inc(captured, 1);
            } else {
                self.has_down.inc(captured, 1);
            }
        }

        Board {
            board: self.board,
            has_down: self.has_down.clone(),
            has_up: self.has_up.clone(),
        }
    }
    fn get(&self,pos:MovePos) -> Piece {
        match pos{
            MovePos::Board(xy) => self.board[xy.x][xy.y],
            MovePos::Hased(p) => p
        }
    }
    fn set(&mut self,pos:MovePos,item:Piece){
        match pos {
            MovePos::Board(xy) => { self.board[xy.x][xy.y] = item; },
            MovePos::Hased(p) => {
                let val = if let PieceEnum::Empty = p.piece {
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

pub struct KifSearchPattern {
    rect: Option<Vec<Vec<Piece>>>,
    hased: Option<[Hased;2]>
}

pub type KifPat = KifSearchPattern;

pub type TKif = Vec<Move>;
pub struct Kif{
    pub kif:TKif,
    pub(crate) map:Board,
}impl Kif {
    ///from_vec for test
    pub fn t_from_vec(kif:TKif) -> Self {
        Self { kif, map: Board::normal() }
    }
    pub fn with_board(kif:TKif, board:Board) -> Self {
        Self { kif, map: board }
    }
    pub fn new(path:&str) -> Result<Self,Box<dyn std::error::Error>> {
        let (_options,moves) = reader::read_kif(path, reader::Opt::default())?;
        Ok(Self::t_from_vec(moves))
    }
    pub fn search(pat:KifSearchPattern) {
        todo!("struct befor")
    }
}

#[cfg(test)]
mod tests {
    use crate::reader;
    type TestRes<E> = Result<(),Box<E>>;
    #[test] fn test_read() -> TestRes<dyn std::error::Error> {
        let readed = reader::read_kif(r".\data\kif1.kif2", reader::Opt::default())?;
        println!("{:#?}",readed.1);
        Ok(())
    }
}