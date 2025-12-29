use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead,BufReader, Result as IoResult},
    str::Chars
};

use crate::{
    TKif,
    Piece,
    PieceEnum,
    Pos,
    Move,
    MovePos,
};

enum SepMoveRet{
    Board(Move),
    Hased(PieceEnum,Pos)
}

pub struct Opt<'a> {
    pub sep:&'a str,
    pub read_sec:Option<Vec<&'a str>>
}impl Default for Opt<'_> {
    fn default() -> Self {
        Self {
            sep: "：",
            read_sec: None
        }
    }
}impl<'a> Opt<'a> {
    fn open_all(self) -> (&'a str,Option<Vec<&'a str>>) {
        (self.sep,self.read_sec)
    }
}

fn sep_move(m_in:&str,prev_pos:Pos) -> Option<SepMoveRet> {
    // 下準備
    let mut it_c: Chars<'_> = m_in.trim_start().chars();
    it_c.next()?;
    loop {
        match it_c.next(){
            Some(' ') => { break; },
            Some(_) => () ,
            None => { panic!("move_num is not matched"); }
        }
    }
    // 例外で早期リターン(詰み,投了, ...etc)
    let _c0 = it_c.next()?;
    match _c0 {
        '中' => { return None; }, //中断
        '投' => { return None; }, //投了
        '持' => { return None; }, //持将棋
        '千' => { return None; }, //千日手
        '切' => { return None; }, //切れ負け
        '反' => { return None; }, //反則(勝ち/負け)
        '入' => { return None; }, //入玉
        '不' => { return None; }, //不戦(勝/敗)/不詰み
        '詰' => { return None; }, //詰み
        _ => {}
    }
    // 移動先
    let to:Pos;
    {
        match _c0 {
            '▲'|'△' => { let _c0 = it_c.next()?;},
            c => { let _c0 = c; }
        }
        let x:usize = match _c0 {
            '１' => 0,
            '２' => 1,
            '３' => 2,
            '４' => 3,
            '５' => 4,
            '６' => 5,
            '７' => 6,
            '８' => 7,
            '９' => 8,
            '同' => prev_pos.x,
            c => {
                if let Some(x) = c.to_digit(10) {
                    x as usize
                } else {
                    panic!("{}",c);
                }
            }
        };
        let y:usize = match it_c.next()? {
            '一' => 0,
            '二' => 1,
            '三' => 2,
            '四' => 3,
            '五' => 4,
            '六' => 5,
            '七' => 6,
            '八' => 7,
            '九' => 8,
            '　' => prev_pos.y,
            c => {
                if let Some(x) = c.to_digit(10){
                    x as usize
                } else if PieceEnum::_get_piece_heads().contains(&c) {
                    prev_pos.y
                } else {
                    panic!("char: {}",c);
                }
            }
        };
        to = Pos::new(x, y);
    }
    // 移動する駒
    let _piece:PieceEnum;
    {
        use PieceEnum::*;
        _piece = match it_c.next()? {
            '玉' => King,
            '飛' => Hisha,
            '角' => Kaku,
            '金' => Gold,
            '銀' => Silver,
            '桂' => Knight,
            '香' => Spear,
            '歩' => Pawn,
            '成' => { it_c.next()?;Empty },
            '全'|'圭'|'杏'|'と'|'龍'|'竜' => Empty,
            c => panic!("char: {}",c)
        }
    }
    // 装飾子
    let _do_promot: bool = false;
    match it_c.next()? {
        '成' => {
            let _do_promot = true;
            it_c.next()?;
        },
        '打' => { return Some( SepMoveRet::Hased(_piece, to) ); },
        '(' => (),
        '銀'|'桂'|'香' => { it_c.next()?; },
        e => panic!("char: {}",e)
    }
    // 移動元
    let from:MovePos;
    {
        let x:usize = match it_c.next()?.to_digit(10) {
            Some(x @ 1..=9) => (x-1) as usize,
            e => {
                panic!("{:?}",e);
            }
        };
        let y:usize = match it_c.next()?.to_digit(10) {
            Some(y @ 1..=9) => (y-1) as usize,
            e => {
                panic!("{:?}",e);
            }
        };
        from = MovePos::Board(Pos::new(x, y));
    }
    Some(SepMoveRet::Board(Move { from, to, do_promot:_do_promot }))
}

const MOVES_PREV:&str = "手数----指手---------消費時間--";

pub(crate) fn read_kif(path:&str,opt:Opt) -> IoResult<(HashMap<String,String>,TKif)> {
    let (separator,read_sect) = opt.open_all();
    let mut ret:HashMap<String, String> = HashMap::<String,String>::new();
    let mut it = BufReader::new(File::open(path)?).lines();
    let mut last_line:String = String::from("pass");
    for l_res in &mut it {
        let l = l_res?;
        if l.starts_with(MOVES_PREV) || l.trim_start().starts_with("1") {
            last_line = l; break;
        }
        if l.starts_with("#") {
            continue;
        }
        if ! read_sect.clone().unwrap_or_default().iter().any(|item| l.starts_with(item)) {
            continue;
        }
        let (k,v) = l.split_once(separator).unwrap_or_default();
        ret.insert(k.to_string(), v.to_string());
    }
    let mut moves:TKif = Vec::new();
    let mut prev_pos  = Pos::new(1, 1);
    let mut is_down = true;
    use SepMoveRet::*;
    if last_line != MOVES_PREV{
        match sep_move(&last_line,prev_pos) {
            Some(smr) => {
            let m = match smr {
                Board(mo) => mo,
                Hased(piece, to) => {
                    Move {from:MovePos::Hased(Piece::new_b(piece, is_down)),to,do_promot:false}
                }
            };
            prev_pos=m.to;
            moves.push(m);
            },
            None => { return Ok((ret,moves)); }
        }
    }
    for l_res in it {
        is_down = ! is_down;
        let l = l_res?;
        match sep_move(&l.trim_start(),prev_pos) {
        Some(smr) => {
            let m = match smr {
                Board(mo) => mo,
                Hased(piece, to) => {
                    Move {from:MovePos::Hased(Piece::new_b(piece, is_down)),to,do_promot:false}
                }
            };
            prev_pos=m.to;
            moves.push(m);
        },
        None => { return Ok((ret,moves)); }
    }
    }
    Ok((ret,moves))
}