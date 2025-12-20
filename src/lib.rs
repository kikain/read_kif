use std::collections::HashMap;

mod read;

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

pub type TBoard = [[Piece;9];9];
pub struct Board{
    pub board:TBoard,
    pub has_down:HashMap<PieceEnum,i32>,
    pub has_up:HashMap<PieceEnum,i32>,
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
            has_down: HashMap::new(),
            has_up: HashMap::new()
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
            has_down: HashMap::new(),
            has_up: HashMap::new()
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
            // <!> get self piece
            if moving.is_down {
                *self.has_up.entry(captured).or_insert(0) += 1;
            } else {
                *self.has_down.entry(captured).or_insert(0) += 1;
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
                // too if{} to else{}
                if p.is_down {
                    self.has_down.insert(p.piece,self.has_down[&p.piece]+if let PieceEnum::Empty = item.piece {-1} else {1});
                } else {
                    self.has_up.insert(p.piece,self.has_up[&p.piece]-1);
                }
            }
        }
    }
    // さらに別のプリセットがあれば同様に関数を追加できます（compact(), handicap(), ...）
}


pub type TKif = Vec<Move>;
pub struct Kif{
    kif:TKif,
    map:Board,
}impl Kif {
    ///from_vec for test
    pub fn t_from_vec(kif:TKif) -> Self {
        Kif { kif, map: Board::normal() }
    }
    pub fn with_board(kif:TKif, board:Board) -> Self {
        Kif { kif, map: board }
    }
    pub fn new(path:&str) -> Result<Self,Box<dyn std::error::Error>> {
        let (_options,moves) = read::read_kif(path, read::Opt::default())?;
        Ok(Self::t_from_vec(moves))
    }
}

#[cfg(test)]
mod tests {
    use crate::read;
    type TestRes<E> = Result<(),Box<E>>;
    #[test] fn test_read() -> TestRes<dyn std::error::Error> {
        let readed = read::read_kif(r".\data\kif2.kif", read::Opt::default())?;
        println!("{:#?}",readed.1);
        Ok(())
    }
}