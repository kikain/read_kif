use crate::{Move, MovePos, Piece, PieceEnum, Pos, TKif};
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Result as IoResult},
    str::Chars,
};

enum SepMoveRet {
    Board(Move),
    Had(PieceEnum, Pos),
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveObj {
    MoveNum,
    ToX,
    ToY,
    Piece,
    FromX,
    FromY,
    Decorator,
}

pub enum SepMoveError<'a> {
    Invalid(MoveObj, &'a str),
    NotFound(MoveObj, &'a str),
    Io(Box<std::io::Error>),
}
impl fmt::Display for SepMoveError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SepMoveError::*;
        let str = match self {
            Invalid(_, _) => "Invalid".to_string(),
            NotFound(_, _) => "NotFound".to_string(),
            Io(_) => "Io".to_string(),
        };
        write!(f, "{}", str)
    }
}
impl fmt::Debug for SepMoveError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = f.debug_struct("SepMoveError");
        ret.field("type", &self.to_string());
        ret.field(
            "portion",
            match self {
                Self::Invalid(val, ..) => val,
                Self::NotFound(val, ..) => val,
                Self::Io(val) => val,
            },
        );
        if let Self::Invalid(_, m_in) | Self::NotFound(_, m_in) = self {
            ret.field("m_in", m_in);
        }
        ret.finish()
    }
}
impl std::error::Error for SepMoveError<'_> {}

pub struct Opt<'a> {
    pub sep: &'a str,
    pub read_sec: Option<Vec<&'a str>>,
}
impl Default for Opt<'_> {
    fn default() -> Self {
        Self {
            sep: "：",
            read_sec: None,
        }
    }
}
impl<'a> Opt<'a> {
    fn open_all(&self) -> (&'a str, Option<Vec<&'a str>>) {
        (self.sep, self.read_sec.clone())
    }
}

fn get_from_pos<'m_in>(
    it: &mut impl Iterator<Item = char>,
    from: MoveObj,
    m_in: &'m_in str,
) -> Result<usize, SepMoveError<'m_in>> {
    match it
        .next()
        .ok_or_else(|| SepMoveError::NotFound(from, m_in))?
        .to_digit(10)
    {
        Some(val @ 1..=9) => Ok((val - 1) as usize),
        _ => Err(SepMoveError::Invalid(from, m_in)),
    }
}
fn sep_move(m_in: &str, prev_pos: Pos) -> Result<SepMoveRet, SepMoveError<'_>> {
    // 下準備
    use MoveObj::*;
    use SepMoveError::*;
    let mut it_c: Chars<'_> = m_in.trim_start().chars();
    it_c.next();
    loop {
        match it_c.next() {
            Some(' ') => {
                break;
            }
            Some(_) => (),
            None => {
                return Err(NotFound(MoveNum, m_in));
            }
        }
    }
    // 例外で早期リターン(詰み,投了, ...etc)
    let _ch = it_c.next().ok_or_else(|| NotFound(ToX, m_in))?;
    match _ch {
        '中' => {
            return Ok(SepMoveRet::Quit);
        } //中断
        '投' => {
            return Ok(SepMoveRet::Quit);
        } //投了
        '持' => {
            return Ok(SepMoveRet::Quit);
        } //持将棋
        '千' => {
            return Ok(SepMoveRet::Quit);
        } //千日手
        '切' => {
            return Ok(SepMoveRet::Quit);
        } //切れ負け
        '反' => {
            return Ok(SepMoveRet::Quit);
        } //反則(勝ち/負け)
        '入' => {
            return Ok(SepMoveRet::Quit);
        } //入玉
        '不' => {
            return Ok(SepMoveRet::Quit);
        } //不戦(勝/敗)/不詰み
        '詰' => {
            return Ok(SepMoveRet::Quit);
        } //詰み
        _ => {}
    }
    // 移動先
    let to: Pos;
    let mut _piece_ch = None;
    {
        match _ch {
            '▲' | '△' => {
                let _ch = it_c.next().ok_or_else(|| NotFound(ToX, m_in));
            }
            c => {
                let _ch = c;
            }
        }
        let x: usize = match _ch {
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
            _ => {
                return Err(Invalid(ToX, m_in));
            }
        };
        let _ch = it_c.next().ok_or_else(|| NotFound(ToY, m_in))?;
        let y: usize = match _ch {
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
                if let Some(x) = c.to_digit(10) {
                    x as usize
                } else if PieceEnum::_get_piece_heads().contains(&c) {
                    _piece_ch = Some(c);
                    prev_pos.y
                } else {
                    return Err(Invalid(ToY, m_in));
                }
            }
        };
        to = Pos::new(x, y);
    }
    // 移動する駒
    let _piece: PieceEnum;
    {
        let _ch = match _piece_ch {
            None => it_c.next().ok_or_else(|| NotFound(Piece, m_in))?,
            Some(c) => c,
        };
        use PieceEnum::*;
        _piece = match _ch {
            '玉' => King,
            '金' => Gold,
            '銀' => Silver,
            '桂' => Knight,
            '香' => Spear,
            '飛' => Rook,
            '角' => Bishop,
            '歩' => Pawn,
            '成' => {
                it_c.next();
                Empty
            }
            '全' | '圭' | '杏' | 'と' | '龍' | '竜' => Empty,
            _ => {
                return Err(Invalid(Piece, m_in));
            }
        }
    }
    // 装飾子
    let mut do_promotion: bool = false;
    let _ch = it_c.next().ok_or_else(|| NotFound(Decorator, m_in))?;
    match _ch {
        '成' => {
            do_promotion = true;
            it_c.next();
        }
        '打' => {
            return Ok(SepMoveRet::Had(_piece, to));
        }
        '(' => (),
        '銀' | '桂' | '香' => {
            it_c.next();
        }
        _ => {
            return Err(Invalid(Decorator, m_in));
        }
    }
    // 移動元
    let from: MovePos;
    {
        from = MovePos::Board(Pos::new(
            get_from_pos(&mut it_c, FromX, m_in)?,
            get_from_pos(&mut it_c, FromY, m_in)?,
        ));
    }
    Ok(SepMoveRet::Board(Move {
        from,
        to,
        do_promotion,
    }))
}

fn push_move_from_smr(smr: SepMoveRet, is_down: bool, prev_pos: &mut Pos, moves: &mut TKif) {
    match smr {
        SepMoveRet::Board(mo) => {
            *prev_pos = mo.to;
            moves.push(mo);
        }
        SepMoveRet::Had(piece, to) => {
            let m = Move {
                from: MovePos::Had(Piece::new_b(piece, is_down)),
                to,
                do_promotion: false,
            };
            *prev_pos = m.to;
            moves.push(m);
        }
        SepMoveRet::Quit => return,
    }
}
const MOVES_PREV: &str = "手数----指手---------消費時間--";
pub(crate) fn read_kif(path: &str, opt: &Opt) -> IoResult<(HashMap<String, String>, TKif)> {
    let (separator, read_sect) = opt.open_all();
    let mut ret: HashMap<String, String> = HashMap::<String, String>::new();
    let mut it = BufReader::new(File::open(path)?).lines();
    let mut last_line: String = String::from("pass");
    for l_res in &mut it {
        let l = l_res?;
        if l.starts_with(MOVES_PREV) || l.trim_start().starts_with("1") {
            last_line = l;
            break;
        }
        if l.starts_with("#") {
            continue;
        }
        if !read_sect
            .clone()
            .unwrap_or_default()
            .iter()
            .any(|item| l.starts_with(item))
        {
            continue;
        }
        let (k, v) = l.split_once(separator).unwrap_or_default();
        ret.insert(k.to_string(), v.to_string());
    }
    let mut moves: TKif = Vec::new();
    let mut prev_pos = Pos::new(1, 1);
    let mut is_down = true;
    if last_line != MOVES_PREV {
        match sep_move(&last_line, prev_pos) {
            Ok(smr) => {
                push_move_from_smr(smr, is_down, &mut prev_pos, &mut moves);
            }
            Err(_) => {
                return Ok((ret, moves));
            }
        }
    }
    for l_res in it {
        is_down = !is_down;
        let l = l_res?;
        match sep_move(&l.trim_start(), prev_pos) {
            Ok(smr) => {
                push_move_from_smr(smr, is_down, &mut prev_pos, &mut moves);
            }
            Err(_) => {
                return Ok((ret, moves));
            }
        }
    }
    Ok((ret, moves))
}
