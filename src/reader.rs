use crate::{Move, MovePos, Piece, PieceEnum, Pos, TKif};
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, Result as IoResult},
};

enum ParsedMove {
    Board(Move),
    InHand(PieceEnum, Pos),
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
impl fmt::Display for MoveObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <MoveObj as fmt::Debug>::fmt(self, f)
    }
}

#[derive(Copy, Clone)]
pub enum ParseErrorKind {
    NotFound,
    Invalid,
    Other(io::ErrorKind),
}
impl From<io::ErrorKind> for ParseErrorKind {
    fn from(value: io::ErrorKind) -> Self {
        use io::ErrorKind as Kind;
        match value {
            Kind::NotFound => Self::NotFound,
            Kind::InvalidData => Self::Invalid,
            other => Self::Other(other),
        }
    }
}
impl From<ParseErrorKind> for io::ErrorKind {
    fn from(value: ParseErrorKind) -> Self {
        use ParseErrorKind as Kind;
        match value {
            Kind::NotFound => Self::NotFound,
            Kind::Invalid => Self::InvalidData,
            Kind::Other(kind) => kind,
        }
    }
}

pub struct ParseError {
    kind: ParseErrorKind,
    obj: MoveObj,
    line: (usize, String),
}
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseMoveEData")
            .field("obj", &self.obj)
            .field("line_num", &self.line.0)
            .field("line_str", &self.line.1)
            .finish()
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A parse error occurred on line {}, type {}, content {} while parsing Move.",
            self.line.0, self.obj, self.line.1
        )
    }
}
impl std::error::Error for ParseError {}
impl From<ParseError> for io::Error {
    fn from(value: ParseError) -> Self {
        Self::new(value.kind.into(), value)
    }
}
impl ParseError {
    /// new for parse_move.
    pub(self) fn new_parse(kind: ParseErrorKind, obj: MoveObj, content: String) -> Self {
        Self {
            kind,
            obj,
            line: (0, content),
        }
    }
    /// add line_num for read_kif.(for output)
    pub(self) fn finish(mut self, line_num: usize) -> io::Error {
        assert_eq!(self.line.0, 0);
        self.line.0 = line_num;
        self.into()
    }
}
type ParseResult<T> = Result<T, ParseError>;

pub struct Opt<'a> {
    pub sep: &'static str,
    pub read_sect: Option<Vec<&'a str>>,
}
impl Default for Opt<'_> {
    fn default() -> Self {
        Self {
            sep: "：",
            read_sect: None,
        }
    }
}
impl<'a> Opt<'a> {
    fn open_all(&self) -> (&'static str, Option<Vec<&'a str>>) {
        (self.sep, self.read_sect.clone())
    }
}
fn get_from_pos(
    it: &mut impl Iterator<Item = char>,
    obj: MoveObj,
    m_in: &str,
) -> ParseResult<usize> {
    use self::ParseErrorKind as Kind;
    match it
        .next()
        .ok_or_else(|| ParseError::new_parse(Kind::NotFound, obj, m_in.to_owned()))?
        .to_digit(10)
    {
        Some(val @ 1..=9) => Ok((val - 1) as usize),
        _ => Err(ParseError::new_parse(Kind::Invalid, obj, m_in.to_owned())),
    }
}
fn parse_move(m_in: &str, prev_pos: Pos) -> ParseResult<ParsedMove> {
    // 下準備
    use self::{MoveObj as Obj, ParseErrorKind as Kind};
    const FULLWIDTH_SPACE: char = '　';
    let mut it = m_in.trim_start().chars().peekable();
    // 手数の次の空白までスキップ
    while it.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        it.next();
    }
    while it.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        it.next();
    }
    // 例外で早期リターン(詰み,投了, .etc)
    {
        let _ch = it
            .peek()
            .ok_or_else(|| ParseError::new_parse(Kind::NotFound, Obj::ToX, m_in.to_owned()))?;
        match _ch {
            '中' => {
                return Ok(ParsedMove::Quit);
            } //中断
            '投' => {
                return Ok(ParsedMove::Quit);
            } //投了
            '持' => {
                return Ok(ParsedMove::Quit);
            } //持将棋
            '千' => {
                return Ok(ParsedMove::Quit);
            } //千日手
            '切' => {
                return Ok(ParsedMove::Quit);
            } //切れ負け
            '反' => {
                return Ok(ParsedMove::Quit);
            } //反則(勝ち/負け)
            '入' => {
                return Ok(ParsedMove::Quit);
            } //入玉
            '不' => {
                return Ok(ParsedMove::Quit);
            } //不戦(勝/敗)/不詰み
            '詰' => {
                return Ok(ParsedMove::Quit);
            } //詰み
            _ => {}
        }
    }
    if matches!(it.peek(), Some('▲') | Some('△')) {
        it.next();
    }
    // 移動先
    let to: Pos;
    let mut _piece_ch = None;
    {
        let _ch = it
            .next()
            .ok_or_else(|| ParseError::new_parse(Kind::NotFound, Obj::ToX, m_in.to_owned()))?;
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
                return Err(ParseError::new_parse(
                    Kind::Invalid,
                    Obj::ToX,
                    m_in.to_owned(),
                ));
            }
        };
        let _ch = it
            .next()
            .ok_or_else(|| ParseError::new_parse(Kind::NotFound, Obj::ToY, m_in.to_owned()))?;
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
            FULLWIDTH_SPACE => prev_pos.y,
            c => {
                if let Some(x) = c.to_digit(10) {
                    x as usize
                } else if PieceEnum::get_piece_heads().contains(&c) {
                    _piece_ch = Some(c);
                    prev_pos.y
                } else {
                    return Err(ParseError::new_parse(
                        Kind::Invalid,
                        Obj::ToY,
                        m_in.to_owned(),
                    ));
                }
            }
        };
        to = Pos::new(x, y);
    }
    // 移動する駒
    let _piece: PieceEnum;
    {
        let _ch = match _piece_ch {
            None => it.next().ok_or_else(|| {
                ParseError::new_parse(Kind::NotFound, Obj::Piece, m_in.to_owned())
            })?,
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
                it.next();
                Empty
            }
            '全' | '圭' | '杏' | 'と' | '龍' | '竜' => Empty,
            _ => {
                return Err(ParseError::new_parse(
                    Kind::Invalid,
                    Obj::Piece,
                    m_in.to_owned(),
                ));
            }
        }
    }
    // 装飾子
    let mut do_promotion: bool = false;
    let _ch = it
        .next()
        .ok_or_else(|| ParseError::new_parse(Kind::NotFound, Obj::Decorator, m_in.to_owned()))?;
    match _ch {
        '成' => {
            do_promotion = true;
            it.next(); // '('のスルー用
        }
        '打' => {
            return Ok(ParsedMove::InHand(_piece, to));
        }
        '(' => (),
        '銀' | '桂' | '香' => {
            it.next();
        }
        _ => {
            return Err(ParseError::new_parse(
                Kind::Invalid,
                Obj::Decorator,
                m_in.to_owned(),
            ));
        }
    }
    let from = MovePos::Board(Pos::new(
        get_from_pos(&mut it, Obj::FromX, m_in)?,
        get_from_pos(&mut it, Obj::FromY, m_in)?,
    ));
    Ok(ParsedMove::Board(Move {
        from,
        to,
        do_promotion,
    }))
}

fn push_move_from_smr(smr: ParsedMove, is_down: bool, prev_pos: &mut Pos, moves: &mut TKif) {
    match smr {
        ParsedMove::Board(mo) => {
            *prev_pos = mo.to;
            moves.push(mo);
        }
        ParsedMove::InHand(piece, to) => {
            let m = Move {
                from: MovePos::Had(Piece::new(piece, is_down, false)),
                to,
                do_promotion: false,
            };
            *prev_pos = m.to;
            moves.push(m);
        }
        ParsedMove::Quit => return,
    }
}
/*
共通処理: 1行分の指し手文字列を解析してmovesに追加する。
true を返すと続行、false を返すと処理を中止（投了や解析エラー）する。
*/
fn process_move_line(
    m_in: &str,
    prev_pos: &mut Pos,
    is_down: bool,
    moves: &mut TKif,
) -> ParseResult<bool> {
    match parse_move(m_in, *prev_pos) {
        Ok(smr) => {
            if let ParsedMove::Quit = smr {
                return Ok(false);
            }
            push_move_from_smr(smr, is_down, prev_pos, moves);
            Ok(true)
        }
        Err(e) => Err(e),
    }
}
const MOVES_PREV: &str = "手数----指手---------消費時間--";
pub fn read_kif(path: &str, opt: &Opt) -> IoResult<(HashMap<String, String>, TKif)> {
    use io::{BufRead, BufReader};
    let (separator, read_sect) = opt.open_all();
    let mut ret: HashMap<String, String> = HashMap::new();
    let mut it = BufReader::new(File::open(path)?).lines().enumerate();
    let mut last_line: String = String::from("pass");
    for (_, l_res) in &mut it {
        let l = l_res?;
        if l.starts_with(MOVES_PREV) || l.trim_start().starts_with("1") {
            last_line = l;
            break;
        }
        if l.starts_with("#") {
            continue;
        }
        if let Some(sections) = read_sect.as_ref() {
            if !sections.iter().any(|item| l.starts_with(item)) {
                continue;
            }
        } else {
            continue;
        }
        let (k, v) = l.split_once(separator).unwrap_or_default();
        ret.insert(k.to_string(), v.to_string());
    }
    let mut moves: TKif = Vec::new();
    let mut prev_pos = Pos::new(1, 1);
    let mut is_down = true;
    if last_line != MOVES_PREV {
        if !process_move_line(&last_line, &mut prev_pos, is_down, &mut moves)? {
            return Ok((ret, moves));
        }
    }
    for (line_num, l_res) in it {
        is_down = !is_down;
        let l = l_res?;
        if !process_move_line(&l, &mut prev_pos, is_down, &mut moves)
            .map_err(|e| e.finish(line_num))?
        {
            return Ok((ret, moves));
        }
    }
    Ok((ret, moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board_move_basic() {
        let prev = Pos::new(0, 0);
        match parse_move("1 ７六歩(77)", prev).expect("parse failed") {
            ParsedMove::Board(m) => {
                assert_eq!(m.to.x, 6);
                assert_eq!(m.to.y, 5);
                match m.from {
                    MovePos::Board(p) => {
                        assert_eq!(p.x, 6);
                        assert_eq!(p.y, 6);
                    }
                    _ => panic!("expected board-from position"),
                }
                assert!(!m.do_promotion);
            }
            _ => panic!("expected board move"),
        }
    }

    #[test]
    fn parse_inhand_move() {
        let prev = Pos::new(0, 0);
        match parse_move("1 ５五歩打", prev).expect("parse failed") {
            ParsedMove::InHand(piece, to) => {
                assert_eq!(piece, PieceEnum::Pawn);
                assert_eq!(to.x, 4);
                assert_eq!(to.y, 4);
            }
            _ => panic!("expected in-hand move"),
        }
    }

    #[test]
    fn parse_quit_move() {
        let prev = Pos::new(0, 0);
        let res = parse_move("1 投了", prev).expect("should parse as quit");
        match res {
            ParsedMove::Quit => {}
            _ => panic!("expected Quit"),
        }
    }

    #[test]
    fn process_move_line_appends_and_updates_prev() {
        let mut prev = Pos::new(0, 0);
        let mut moves: TKif = Vec::new();
        let cont =
            process_move_line("1 ７六歩(77)", &mut prev, true, &mut moves).expect("process failed");
        assert!(cont);
        assert_eq!(moves.len(), 1);
        assert_eq!(prev.x, 6);
        assert_eq!(prev.y, 5);
    }

    #[test]
    fn parse_invalid_returns_err() {
        let prev = Pos::new(0, 0);
        assert!(parse_move("1 @@@", prev).is_err());
    }
}
