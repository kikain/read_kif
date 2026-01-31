extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
mod reader {

    // さらに別のプリセットがあれば同様に関数を追加できます（compact(), handicap(), ...）

    /*.unwrap_or_default()*/
    use crate::Move;
    use crate::MovePos;
    use crate::Piece;
    use crate::PieceEnum;
    use crate::Pos;
    use crate::TKif;
    use std::collections::HashMap;
    use std::fmt;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Result as IoResult;
    use std::str::Chars;
    enum SepMoveRet { Board(Move), InHand(PieceEnum, Pos), Quit }
    enum MoveObj { MoveNum, ToX, ToY, Piece, FromX, FromY, Decorator }
    #[attr = AutomaticallyDerived]
    impl ::core::fmt::Debug for MoveObj {
        #[attr = Inline(Hint)]
        fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
               ->
               ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                                              match self {
                                                  MoveObj::MoveNum => "MoveNum",
                                                  MoveObj::ToX => "ToX",
                                                  MoveObj::ToY => "ToY",
                                                  MoveObj::Piece => "Piece",
                                                  MoveObj::FromX => "FromX",
                                                  MoveObj::FromY => "FromY",
                                                  MoveObj::Decorator => "Decorator",
                                              })
        }
    }
    #[attr = AutomaticallyDerived]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    unsafe impl ::core::clone::TrivialClone for MoveObj {}
    #[attr = AutomaticallyDerived]
    impl ::core::clone::Clone for MoveObj {
        #[attr = Inline(Hint)]
        fn clone(&self) -> MoveObj { *self }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::Copy for MoveObj {}
    enum SepMoveError {
        Invalid(MoveObj, String),
        NotFound(MoveObj, String),
        Io(Box<std::io::Error>),
    }
    impl fmt::Display for SepMoveError {
        fn fmt(&self, f: &'_ mut fmt::Formatter<'_>)
               ->
               fmt::Result {
            use self::SepMoveError::*;
            let str =
                match self {
                    Invalid(..) => "Invalid".to_string(),
                    NotFound(..) => "NotFound".to_string(),
                    Io(..) => "Io".to_string(),
                };
            f.write_fmt({
                super
                let args = (&str,);
                super
                let args = [format_argument::new_display(args.0)];
                unsafe { format_arguments::new(b"\xc0\x00", &args) }
            })
        }
    }
    impl fmt::Debug for SepMoveError {
        fn fmt(&self, f: &'_ mut fmt::Formatter<'_>)
               ->
               fmt::Result {
            let mut ret = f.debug_struct("SepMoveError");
            ret.field("type", &self.to_string());
            ret.field("portion",
                      match self {
                          Self::Invalid(val, ..) => val,
                          Self::NotFound(val, ..) => val,
                          Self::Io(val) => val,
                      });
            if let Self::Invalid(_, m_in) | Self::NotFound(_, m_in) = self {
                ret.field("m_in", m_in);
            }
            ret.finish()
        }
    }
    impl std::error::Error for SepMoveError {}
    struct Opt<'a> {
        sep: &'a str,
        read_sect: Option<Vec<&'a str>>,
    }
    impl Default for Opt<'_> {
        fn default() -> Self { Self { sep: "\u{ff1a}", read_sect: None } }
    }
    impl<'a> Opt<'a> {
        fn open_all(&self)
                    ->
                    (&'a str,
                     Option<Vec<&'a str>>) { (self.sep, self.read_sect.clone()) }
    }
    fn get_from_pos < impl Iterator<Item=char> > (it:
    & '_ mut impl Iterator<Item=char>, from: MoveObj, m_in: & '_ str)
    -> Result<usize, SepMoveError> where
    impl Iterator<Item=char>: Iterator<Item =
    char> {
match match branch(it.next().ok_or_else( | |
SepMoveError::NotFound(from, m_in.to_owned()))) {
Break {  0: residual } => # [allow(unreachable_code)]
return from_residual(residual),
Continue {  0: val } => # [allow(unreachable_code)]
val,
}.to_digit(10) {
Some(val @ 1...9) => Ok((val - 1) as usize),
_ => Err(SepMoveError::Invalid(from, m_in.to_owned())),
}
}
    fn sep_move(m_in: &'_ str, prev_pos: Pos)
                ->
                Result<SepMoveRet,
                    SepMoveError> {
        use self::MoveObj::*;
        use self::SepMoveError::*;
        let mut it_c: Chars = m_in.trim_start().chars();
        it_c.next();
        loop {
            match it_c.next() {
                Some(' ') => { break; }
                Some(_) => (),
                None => { return Err(NotFound(MoveNum, m_in.to_owned())); }
            }
        }
        let _ch =
            match branch(it_c.next().ok_or_else(||
                NotFound(ToX, m_in.to_owned()))) {
                Break { 0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
                Continue { 0: val } => #[allow(unreachable_code)]
                val,
            };
        match _ch {
            '\u{4e2d}' => { return Ok(SepMoveRet::Quit); }
            '\u{6295}' => { return Ok(SepMoveRet::Quit); }
            '\u{6301}' => { return Ok(SepMoveRet::Quit); }
            '\u{5343}' => { return Ok(SepMoveRet::Quit); }
            '\u{5207}' => { return Ok(SepMoveRet::Quit); }
            '\u{53cd}' => { return Ok(SepMoveRet::Quit); }
            '\u{5165}' => { return Ok(SepMoveRet::Quit); }
            '\u{4e0d}' => { return Ok(SepMoveRet::Quit); }
            '\u{8a70}' => { return Ok(SepMoveRet::Quit); }
            _ => {}
        }
        let to: Pos;
        let mut _piece_ch = None;
        {
            match _ch {
                '\u{25b2}' | '\u{25b3}' => {
                    let _ch =
                        it_c.next().ok_or_else(|| NotFound(ToX, m_in.to_owned()));
                }
                c => { let _ch = c; }
            }
            let x: usize =
                match _ch {
                    '\u{ff11}' => 0,
                    '\u{ff12}' => 1,
                    '\u{ff13}' => 2,
                    '\u{ff14}' => 3,
                    '\u{ff15}' => 4,
                    '\u{ff16}' => 5,
                    '\u{ff17}' => 6,
                    '\u{ff18}' => 7,
                    '\u{ff19}' => 8,
                    '\u{540c}' => prev_pos.x,
                    _ => { return Err(Invalid(ToX, m_in.to_owned())); }
                };
            let _ch =
                match branch(it_c.next().ok_or_else(||
                    NotFound(ToY, m_in.to_owned()))) {
                    Break { 0: residual } => #[allow(unreachable_code)]
                    return from_residual(residual),
                    Continue { 0: val } => #[allow(unreachable_code)]
                    val,
                };
            let y: usize =
                match _ch {
                    '\u{4e00}' => 0,
                    '\u{4e8c}' => 1,
                    '\u{4e09}' => 2,
                    '\u{56db}' => 3,
                    '\u{4e94}' => 4,
                    '\u{516d}' => 5,
                    '\u{4e03}' => 6,
                    '\u{516b}' => 7,
                    '\u{4e5d}' => 8,
                    '\u{3000}' => prev_pos.y,
                    c => {
                        if let Some(x) = c.to_digit(10) {
                            x as usize
                        } else if PieceEnum::_get_piece_heads().contains(&c) {
                            _piece_ch = Some(c);
                            prev_pos.y
                        } else { return Err(Invalid(ToY, m_in.to_owned())); }
                    }
                };
            to = Pos::new(x, y);
        }
        let _piece: PieceEnum;
        {
            let _ch =
                match _piece_ch {
                    None =>
                        match branch(it_c.next().ok_or_else(||
                            NotFound(Piece, m_in.to_owned()))) {
                            Break { 0: residual } => #[allow(unreachable_code)]
                            return from_residual(residual),
                            Continue { 0: val } => #[allow(unreachable_code)]
                            val,
                        },
                    Some(c) => c,
                };
            use PieceEnum::*;
            _piece =
                match _ch {
                    '\u{7389}' => King,
                    '\u{91d1}' => Gold,
                    '\u{9280}' => Silver,
                    '\u{6842}' => Knight,
                    '\u{9999}' => Spear,
                    '\u{98db}' => Rook,
                    '\u{89d2}' => Bishop,
                    '\u{6b69}' => Pawn,
                    '\u{6210}' => {
                        it_c.next();
                        Empty
                    }
                    '\u{5168}' | '\u{572d}' | '\u{674f}' | '\u{3068}' |
                    '\u{9f8d}' | '\u{7adc}' => Empty,
                    _ => { return Err(Invalid(Piece, m_in.to_owned())); }
                }
        }
        let mut do_promotion: bool = false;
        let _ch =
            match branch(it_c.next().ok_or_else(||
                NotFound(Decorator, m_in.to_owned()))) {
                Break { 0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
                Continue { 0: val } => #[allow(unreachable_code)]
                val,
            };
        match _ch {
            '\u{6210}' => {
                do_promotion = true;
                it_c.next();
            }
            '\u{6253}' => { return Ok(SepMoveRet::InHand(_piece, to)); }
            '(' => (),
            '\u{9280}' | '\u{6842}' | '\u{9999}' => { it_c.next(); }
            _ => { return Err(Invalid(Decorator, m_in.to_owned())); }
        }
        let from =
            MovePos::Board(Pos::new(match branch(get_from_pos(&mut it_c,
                                                              FromX, m_in)) {
                Break { 0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
                Continue { 0: val } => #[allow(unreachable_code)]
                val,
            },
                                    match branch(get_from_pos(&mut it_c, FromY, m_in)) {
                                        Break { 0: residual } => #[allow(unreachable_code)]
                                        return from_residual(residual),
                                        Continue { 0: val } => #[allow(unreachable_code)]
                                        val,
                                    }));
        Ok(SepMoveRet::Board(Move { from, to, do_promotion }))
    }
    fn push_move_from_smr(smr: SepMoveRet, is_down: bool,
                          prev_pos: &'_ mut Pos,
                          moves:
                          &'_ mut TKif) {
        match smr {
            SepMoveRet::Board(mo) => {
                *prev_pos = mo.to;
                moves.push(mo);
            }
            SepMoveRet::InHand(piece, to) => {
                let m =
                    Move {
                        from: MovePos::Had(Piece::new(piece, is_down, false)),
                        to,
                        do_promotion: false,
                    };
                *prev_pos = m.to;
                moves.push(m);
            }
            SepMoveRet::Quit => return,
        }
    }
    fn process_move_line(m_in: &'_ str, prev_pos: &'_ mut Pos, is_down: bool,
                         moves: &'_ mut TKif)
                         ->
                         bool {
        match sep_move(m_in, *prev_pos) {
            Ok(smr) => {
                if let SepMoveRet::Quit = smr { return false; }
                push_move_from_smr(smr, is_down, prev_pos, moves);
                true
            }
            Err(_) => false,
        }
    }
    const MOVES_PREV: &'_ str =
        "\u{624b}\u{6570}----\u{6307}\u{624b}---------\u{6d88}\u{8cbb}\u{6642}\u{9593}--";
    fn read_kif(path: &'_ str, opt: &'_ Opt<'_>)
                ->
                IoResult<(HashMap<String, String>,
                          TKif)> {
        let (separator, read_sect) = opt.open_all();
        let mut ret: HashMap<String, String> = HashMap < String, String > ::new();
        let mut it =
            BufReader::new(match branch(File::open(path)) {
                Break { 0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
                Continue { 0: val } => #[allow(unreachable_code)]
                val,
            }).lines();
        let mut last_line: String = String::from("pass");
        {
            let _t =
                match into_iter(&mut it) {
                    mut iter =>
                        loop {
                            match next(&mut iter) {
                                None {} => break,
                                Some { 0: l_res } => {
                                    let l =
                                        match branch(l_res) {
                                            Break { 0: residual } => #[allow(unreachable_code)]
                                            return from_residual(residual),
                                            Continue { 0: val } => #[allow(unreachable_code)]
                                            val,
                                        };
                                    if l.starts_with(MOVES_PREV) ||
                                        l.trim_start().starts_with("1") {
                                        last_line = l;
                                        break;
                                    }
                                    if l.starts_with("#") { continue; }
                                    if !read_sect.clone().unwrap_or_default().iter().any(|item|
                                        l.starts_with(item)) {
                                        continue;
                                    }
                                    let (k, v) = l.split_once(separator).unwrap_or_default();
                                    ret.insert(k.to_string(), v.to_string());
                                }
                            }
                        },
                };
            _t
        };
        let mut moves: TKif = Vec::new();
        let mut prev_pos = Pos::new(1, 1);
        let mut is_down = true;
        if last_line != MOVES_PREV {
            if !process_move_line(&last_line, &mut prev_pos, is_down,
                                  &mut moves) {
                return Ok((ret, moves));
            }
        }
        {
            let _t =
                match into_iter(it) {
                    mut iter =>
                        loop {
                            match next(&mut iter) {
                                None {} => break,
                                Some { 0: l_res } => {
                                    is_down = !is_down;
                                    let l =
                                        match branch(l_res) {
                                            Break { 0: residual } => #[allow(unreachable_code)]
                                            return from_residual(residual),
                                            Continue { 0: val } => #[allow(unreachable_code)]
                                            val,
                                        };
                                    if !process_move_line(&l, &mut prev_pos, is_down,
                                                          &mut moves) {
                                        return Ok((ret, moves));
                                    }
                                }
                            }
                        },
                };
            _t
        };
        Ok((ret, moves))
    }
}
mod search {
    use crate::Board;
    use crate::Had;
    use crate::Piece;
    use crate::TBoard;
    enum KifPatPiece { Ignore, Handle(Piece) }
    #[attr = AutomaticallyDerived]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    unsafe impl ::core::clone::TrivialClone for KifPatPiece {}
    #[attr = AutomaticallyDerived]
    impl ::core::clone::Clone for KifPatPiece {
        #[attr = Inline(Hint)]
        fn clone(&self)
                 ->
                 KifPatPiece {
            let _: ::core::clone::AssertParamIsClone<Piece>;
            *self
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::Copy for KifPatPiece {}
    #[attr = AutomaticallyDerived]
    impl ::core::fmt::Debug for KifPatPiece {
        #[attr = Inline(Hint)]
        fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
               ->
               ::core::fmt::Result {
            match self {
                KifPatPiece::Ignore =>
                    ::core::fmt::Formatter::write_str(f, "Ignore"),
                KifPatPiece::Handle(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                                                                      "Handle", &__self_0),
            }
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::StructuralPartialEq for KifPatPiece {}
    #[attr = AutomaticallyDerived]
    impl ::core::cmp::PartialEq for KifPatPiece {
        #[attr = Inline(Hint)]
        fn eq(&self, other: &'_ KifPatPiece)
              ->
              bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr &&
                match (self, other) {
                    (KifPatPiece::Handle(__self_0),
                        KifPatPiece::Handle(__arg1_0)) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::cmp::Eq for KifPatPiece {
        #[attr = Inline(Hint)]
        #[attr = Doc(DocAttribute {aliases: [],
        inline: [],
        cfg: [],
        auto_cfg: [],
        auto_cfg_change: [],
        test_attrs: []})]
        #[attr = Coverage(Off)]
        fn assert_receiver_is_total_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<Piece>;
        }
    }
    impl PartialEq<Piece> for KifPatPiece {
        fn eq(&self, other: &'_ Piece)
              ->
              bool {
            if let KifPatPiece::Ignore = self {
                true
            } else {
                let KifPatPiece::Handle(self_p) = *self else
                {
                    {
                        ::core::panicking::unreachable_display(&"I previously checked self==KifPatPiece::Ignore but found it here");
                    }
                };
                &self_p == other
            }
        }
    }
    struct Rect {
        start: (u32, u32),
        range: (u32, u32),
    }
    impl Rect {
        const fn new() -> Self { Self { start: (0, 0), range: (0, 0) } }
        const fn with_end(end: (u32, u32))
                          -> Self { Self { start: (0, 0), range: end } }
        const fn with_start_end(start: (u32, u32), end: (u32, u32))
                                ->
                                Self {
            Self { start, range: (end.0 - start.0, end.1 - start.1) }
        }
        const fn is_inside(&self, x: u32, y: u32)
                           ->
                           bool {
            if self.start.0 <= x && x <= self.start.0 + self.range.0 {
                if self.start.1 <= y && y <= self.start.1 + self.range.1 {
                    return true;
                }
            }
            false
        }
        const fn is_inside_x(&self, val: u32)
                             ->
                             bool {
            if self.start.0 <= val && val <= self.start.0 + self.range.0 {
                return true;
            }
            false
        }
        const fn is_inside_y(&self, val: u32)
                             ->
                             bool {
            if self.start.1 <= val && val <= self.start.1 + self.range.1 {
                return true;
            }
            false
        }
    }
    struct KifPatBoard {
        board: [[KifPatPiece; 9]; 9],
        some_start: (usize, usize),
    }
    #[attr = AutomaticallyDerived]
    impl ::core::fmt::Debug for KifPatBoard {
        #[attr = Inline(Hint)]
        fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
               ->
               ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(f,
                                                               "KifPatBoard", "board", &self.board, "some_start",
                                                               &&self.some_start)
        }
    }
    #[attr = AutomaticallyDerived]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    unsafe impl ::core::clone::TrivialClone for KifPatBoard {}
    #[attr = AutomaticallyDerived]
    impl ::core::clone::Clone for KifPatBoard {
        #[attr = Inline(Hint)]
        fn clone(&self)
                 ->
                 KifPatBoard {
            let _: ::core::clone::AssertParamIsClone<[[KifPatPiece; 9]; 9]>;
            let _: ::core::clone::AssertParamIsClone<(usize, usize)>;
            *self
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::Copy for KifPatBoard {}
    impl KifPatBoard {
        const fn new()
            ->
            Self {
            Self { board: [[KifPatPiece::Ignore; 9]; 9], some_start: (9, 9) }
        }
        const fn with_board(board: TBoard)
                            ->
                            Self {
            let mut temp = [[KifPatPiece::Ignore; 9]; 9];
            let mut ix: usize = 0;
            loop {
                if ix <= 8 {
                    let mut iy: usize = 0;
                    loop {
                        if iy <= 8 {
                            temp[ix][iy] = KifPatPiece::Handle(board[ix][iy]);
                            iy += 1;
                        } else { break; }
                    }
                    ix += 1;
                } else { break; }
            }
            Self { board: temp, some_start: (0, 0) }
        }
        const fn with_board_area(board: TBoard, area: Rect)
                                 ->
                                 Self {
            let mut temp = [[KifPatPiece::Ignore; 9]; 9];
            let mut ix: usize = 0;
            let mut some_start: (usize, usize) = (9, 9);
            loop {
                if ix <= 8 {
                    if !area.is_inside_x(ix as u32) {
                        temp[ix] = [KifPatPiece::Ignore; 9];
                        ix += 1;
                        continue;
                    } else if some_start.0 == 9 { some_start.0 = ix; }
                    let mut iy: usize = 0;
                    loop {
                        if iy <= 8 {
                            if area.is_inside_y(iy as u32) {
                                temp[ix][iy] = KifPatPiece::Ignore;
                                iy += 1;
                                continue;
                            } else if some_start.1 == 9 { some_start.1 = iy; }
                            temp[ix][iy] = KifPatPiece::Handle(board[ix][iy]);
                            iy += 1;
                        } else { break; }
                    }
                    ix += 1;
                } else { break; }
            }
            Self { board: temp, some_start }
        }
        const fn is_all_ignored(&self)
                                ->
                                bool {
            let mut ix: usize = 0;
            loop {
                if ix <= 8 {
                    let mut iy: usize = 0;
                    loop {
                        if iy <= 8 {
                            if let KifPatPiece::Ignore = self.board[ix][iy] {} else { return false; }
                            iy += 1
                        } else { break; }
                    }
                    ix += 1;
                } else { break; }
            }
            true
        }
        fn search_all(&self, target: TBoard)
                      ->
                      bool {
            let mut ix: usize = 0;
            loop {
                if ix <= 8 {
                    let mut iy: usize = 0;
                    loop {
                        if iy <= 8 {
                            if self.board[ix][iy] != target[ix][iy] { return false; }
                            iy += 1;
                        } else { break; }
                    }
                    ix += 1;
                } else { break; }
            }
            true
        }
        fn search(&self, target: TBoard)
                  ->
                  bool {
            if self.some_start.0 == 9 && self.some_start.1 == 9 {
                true
            } else {
                if self.board[self.some_start.0][self.some_start.1] !=
                    target[self.some_start.0][self.some_start.1] {
                    false
                } else { self.search_all(target) }
            }
        }
    }
    struct KifPatHad {
        king: Option<u32>,
        gold: Option<u32>,
        silver: Option<u32>,
        knight: Option<u32>,
        spear: Option<u32>,
        rook: Option<u32>,
        bishop: Option<u32>,
        pawn: Option<u32>,
    }
    #[attr = AutomaticallyDerived]
    impl ::core::fmt::Debug for KifPatHad {
        #[attr = Inline(Hint)]
        fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
               ->
               ::core::fmt::Result {
            let names: &'static _ =
                &["king", "gold", "silver", "knight", "spear", "rook",
                    "bishop", "pawn"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.king, &self.gold, &self.silver, &self.knight,
                    &self.spear, &self.rook, &self.bishop, &&self.pawn];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "KifPatHad",
                                                               names, values)
        }
    }
    #[attr = AutomaticallyDerived]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    unsafe impl ::core::clone::TrivialClone for KifPatHad {}
    #[attr = AutomaticallyDerived]
    impl ::core::clone::Clone for KifPatHad {
        #[attr = Inline(Hint)]
        fn clone(&self)
                 ->
                 KifPatHad {
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            let _: ::core::clone::AssertParamIsClone<Option<u32>>;
            *self
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::Copy for KifPatHad {}
    impl KifPatHad {
        const fn new()
            ->
            Self {
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
        const fn is_all_ignored(&self)
                                ->
                                bool {
            if self.king.is_none() && self.gold.is_none() &&
                self.silver.is_none() && self.knight.is_none() &&
                self.spear.is_none() && self.rook.is_none() &&
                self.bishop.is_none() && self.pawn.is_none() {
                true
            } else { false }
        }
        const fn to_option(self)
                           ->
                           Option<KifPatHad> {
            if self.is_all_ignored() { None } else { Some(self) }
        }
        const fn had_eq(me: Option<u32>, other: u32)
                        -> bool {
            match me {
                None => true,
                Some(val) => val == other,
            }
        }
        const fn search(&self, target: Had)
                        ->
                        bool {
            if self.is_all_ignored() {
                true
            } else {
                Self::had_eq(self.king, target.king) &&
                    Self::had_eq(self.gold, target.gold) &&
                    Self::had_eq(self.silver, target.silver) &&
                    Self::had_eq(self.knight, target.knight) &&
                    Self::had_eq(self.spear, target.spear) &&
                    Self::had_eq(self.rook, target.rook) &&
                    Self::had_eq(self.bishop, target.bishop) &&
                    Self::had_eq(self.pawn, target.pawn)
            }
        }
    }
    impl Default for KifPatHad {
        fn default() -> Self { Self::new() }
    }
    impl From<Had> for KifPatHad {
        fn from(had: Had)
                ->
                Self {
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
    struct KifSearchPattern {
        board: Option<KifPatBoard>,
        had_up: Option<KifPatHad>,
        had_down: Option<KifPatHad>,
    }
    #[attr = AutomaticallyDerived]
    impl ::core::fmt::Debug for KifSearchPattern {
        #[attr = Inline(Hint)]
        fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
               ->
               ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f,
                                                               "KifSearchPattern", "board", &self.board, "had_up",
                                                               &self.had_up, "had_down", &&self.had_down)
        }
    }
    #[attr = AutomaticallyDerived]
    impl ::core::marker::Copy for KifSearchPattern {}
    #[attr = AutomaticallyDerived]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    unsafe impl ::core::clone::TrivialClone for KifSearchPattern {}
    #[attr = AutomaticallyDerived]
    impl ::core::clone::Clone for KifSearchPattern {
        #[attr = Inline(Hint)]
        fn clone(&self)
                 ->
                 KifSearchPattern {
            let _: ::core::clone::AssertParamIsClone<Option<KifPatBoard>>;
            let _: ::core::clone::AssertParamIsClone<Option<KifPatHad>>;
            let _: ::core::clone::AssertParamIsClone<Option<KifPatHad>>;
            *self
        }
    }
    impl KifSearchPattern {
        const
        DEFAULT:
        Self
        =
            Self { board: None, had_up: None, had_down: None };
        const fn new() -> Self { Self::DEFAULT }
        const fn with_board(board: KifPatBoard)
                            ->
                            Self {
            Self {
                board:
                if board.is_all_ignored() {
                    None
                } else { Some(board) },
                ..Self::DEFAULT
            }
        }
        const fn with_had(had_up: KifPatHad, had_down: KifPatHad)
                          ->
                          Self {
            Self {
                had_up: had_up.to_option(),
                had_down: had_down.to_option(),
                ..Self::DEFAULT
            }
        }
        fn search(&self, target: &'_ Board)
                  ->
                  bool {
            if let Some(board_pat) = self.board {
                if !board_pat.search(target.board) { return false; }
            }
            if let Some(had_up_pat) = self.had_up {
                if !had_up_pat.search(target.had_up) { return false; }
            }
            if let Some(had_down_pat) = self.had_down {
                if !had_down_pat.search(target.had_down) { return false; }
            }
            true
        }
    }
    type KifPat = KifSearchPattern;
}
enum PieceEnum {
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
#[attr = AutomaticallyDerived]
#[attr = Doc(DocAttribute {aliases: [], inline: [], cfg: [], auto_cfg: [],
auto_cfg_change: [], test_attrs: []})]
unsafe impl ::core::clone::TrivialClone for PieceEnum {}
#[attr = AutomaticallyDerived]
impl ::core::clone::Clone for PieceEnum {
    #[attr = Inline(Hint)]
    fn clone(&self) -> PieceEnum { *self }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::Copy for PieceEnum {}
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for PieceEnum {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
                                          match self {
                                              PieceEnum::Empty => "Empty",
                                              PieceEnum::King => "King",
                                              PieceEnum::Gold => "Gold",
                                              PieceEnum::Silver => "Silver",
                                              PieceEnum::Knight => "Knight",
                                              PieceEnum::Spear => "Spear",
                                              PieceEnum::Rook => "Rook",
                                              PieceEnum::Bishop => "Bishop",
                                              PieceEnum::Pawn => "Pawn",
                                          })
    }
}
#[attr = AutomaticallyDerived]
impl ::core::default::Default for PieceEnum {
    #[attr = Inline(Hint)]
    fn default() -> PieceEnum { Self::Empty }
}
#[attr = AutomaticallyDerived]
impl ::core::cmp::Eq for PieceEnum {
    #[attr = Inline(Hint)]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    #[attr = Coverage(Off)]
    fn assert_receiver_is_total_eq(&self) {}
}
#[attr = AutomaticallyDerived]
impl ::core::marker::StructuralPartialEq for PieceEnum {}
#[attr = AutomaticallyDerived]
impl ::core::cmp::PartialEq for PieceEnum {
    #[attr = Inline(Hint)]
    fn eq(&self, other: &'_ PieceEnum)
          ->
          bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
impl PieceEnum {
    fn _get_piece_heads()
        ->
        [char; 15] {
        ['\u{7389}', '\u{91d1}', '\u{9280}', '\u{6842}', '\u{9999}',
            '\u{98db}', '\u{89d2}', '\u{6b69}', '\u{6210}', '\u{5168}',
            '\u{572d}', '\u{674f}', '\u{3068}', '\u{9f8d}', '\u{7adc}']
    }
}
struct Piece {
    piece: PieceEnum,
    is_down: bool,
    is_promoted: bool,
}
#[attr = AutomaticallyDerived]
#[attr = Doc(DocAttribute {aliases: [], inline: [], cfg: [], auto_cfg: [],
auto_cfg_change: [], test_attrs: []})]
unsafe impl ::core::clone::TrivialClone for Piece {}
#[attr = AutomaticallyDerived]
impl ::core::clone::Clone for Piece {
    #[attr = Inline(Hint)]
    fn clone(&self)
             ->
             Piece {
        let _: ::core::clone::AssertParamIsClone<PieceEnum>;
        let _: ::core::clone::AssertParamIsClone<bool>;
        *self
    }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::Copy for Piece {}
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for Piece {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "Piece",
                                                           "piece", &self.piece, "is_down", &self.is_down, "is_promoted",
                                                           &&self.is_promoted)
    }
}
#[attr = AutomaticallyDerived]
impl ::core::default::Default for Piece {
    #[attr = Inline(Hint)]
    fn default()
        ->
        Piece {
        Piece {
            piece: ::core::default::Default::default(),
            is_down: ::core::default::Default::default(),
            is_promoted: ::core::default::Default::default(),
        }
    }
}
#[attr = AutomaticallyDerived]
impl ::core::cmp::Eq for Piece {
    #[attr = Inline(Hint)]
    #[attr = Doc(DocAttribute {aliases: [],
    inline: [],
    cfg: [],
    auto_cfg: [],
    auto_cfg_change: [],
    test_attrs: []})]
    #[attr = Coverage(Off)]
    fn assert_receiver_is_total_eq(&self) {
        let _: ::core::cmp::AssertParamIsEq<PieceEnum>;
        let _: ::core::cmp::AssertParamIsEq<bool>;
    }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::StructuralPartialEq for Piece {}
#[attr = AutomaticallyDerived]
impl ::core::cmp::PartialEq for Piece {
    #[attr = Inline(Hint)]
    fn eq(&self, other: &'_ Piece)
          ->
          bool {
        self.is_down == other.is_down && self.is_promoted == other.is_promoted
            && self.piece == other.piece
    }
}
impl Piece {
    const
    DEFAULT:
    Self
    =
        Self { piece: PieceEnum::Empty, is_down: false, is_promoted: false };
    ///new func for initializing the Board
    const fn new_b(pieces: [PieceEnum; 9], is_down: bool)
                   ->
                   [Self; 9] {
        let mut temp = [Self::new(PieceEnum::Empty, is_down, false); 9];
        let mut i: usize = 1;
        loop {
            if i < 9 {
                temp[i] = Self::new(pieces[i], is_down, false);
                i += 1;
            } else { break; }
        }
        temp
    }
    const fn new(piece: PieceEnum, is_down: bool, is_promoted: bool)
                 -> Self { Piece { piece, is_down, is_promoted } }
}
struct Pos {
    x: usize,
    y: usize,
}
#[attr = AutomaticallyDerived]
#[attr = Doc(DocAttribute {aliases: [], inline: [], cfg: [], auto_cfg: [],
auto_cfg_change: [], test_attrs: []})]
unsafe impl ::core::clone::TrivialClone for Pos {}
#[attr = AutomaticallyDerived]
impl ::core::clone::Clone for Pos {
    #[attr = Inline(Hint)]
    fn clone(&self)
             -> Pos {
        let _: ::core::clone::AssertParamIsClone<usize>;
        *self
    }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::Copy for Pos {}
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for Pos {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "Pos", "x",
                                                           &self.x, "y", &&self.y)
    }
}
impl Pos {
    const fn new(x: usize, y: usize)
                 ->
                 Self {
        if !(x <= 8) { { ::std::rt::begin_panic("x ranges from 0 to 8."); } };
        if !(y <= 8) { { ::std::rt::begin_panic("y ranges from 0 to 8."); } };
        Pos { x, y }
    }
    ///self to MovePos
    const fn to_mp(&self) -> MovePos { MovePos::Board(*self) }
}
enum MovePos { Board(Pos), Had(Piece) }
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for MovePos {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        match self {
            MovePos::Board(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Board",
                                                                  &__self_0),
            MovePos::Had(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Had",
                                                                  &__self_0),
        }
    }
}
#[attr = AutomaticallyDerived]
#[attr = Doc(DocAttribute {aliases: [], inline: [], cfg: [], auto_cfg: [],
auto_cfg_change: [], test_attrs: []})]
unsafe impl ::core::clone::TrivialClone for MovePos {}
#[attr = AutomaticallyDerived]
impl ::core::clone::Clone for MovePos {
    #[attr = Inline(Hint)]
    fn clone(&self)
             ->
             MovePos {
        let _: ::core::clone::AssertParamIsClone<Pos>;
        let _: ::core::clone::AssertParamIsClone<Piece>;
        *self
    }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::Copy for MovePos {}
struct Move {
    from: MovePos,
    to: Pos,
    do_promotion: bool,
}
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for Move {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "Move", "from",
                                                           &self.from, "to", &self.to, "do_promotion", &&self.do_promotion)
    }
}
impl Move {}
struct Had {
    king: u32,
    gold: u32,
    silver: u32,
    knight: u32,
    spear: u32,
    rook: u32,
    bishop: u32,
    pawn: u32,
}
#[attr = AutomaticallyDerived]
impl ::core::fmt::Debug for Had {
    #[attr = Inline(Hint)]
    fn fmt(&self, f: &'_ mut ::core::fmt::Formatter<'_>)
           ->
           ::core::fmt::Result {
        let names: &'static _ =
            &["king", "gold", "silver", "knight", "spear", "rook", "bishop",
                "pawn"];
        let values: &[&dyn ::core::fmt::Debug] =
            &[&self.king, &self.gold, &self.silver, &self.knight, &self.spear,
                &self.rook, &self.bishop, &&self.pawn];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Had", names,
                                                           values)
    }
}
#[attr = AutomaticallyDerived]
#[attr = Doc(DocAttribute {aliases: [], inline: [], cfg: [], auto_cfg: [],
auto_cfg_change: [], test_attrs: []})]
unsafe impl ::core::clone::TrivialClone for Had {}
#[attr = AutomaticallyDerived]
impl ::core::clone::Clone for Had {
    #[attr = Inline(Hint)]
    fn clone(&self)
             -> Had {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[attr = AutomaticallyDerived]
impl ::core::marker::Copy for Had {}
impl Default for Had {
    fn default()
        ->
        Self {
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
    const
    DEFAULT:
    Self
    =
        Had {
            king: 0,
            gold: 0,
            silver: 0,
            knight: 0,
            spear: 0,
            rook: 0,
            bishop: 0,
            pawn: 0,
        };
    const fn get(&self, key: PieceEnum)
                 ->
                 u32 {
        use PieceEnum::*;
        match key {
            Empty => { ::std::rt::begin_panic("had in not \'Empty\'"); }
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
    fn adjust_count(field: &'_ mut u32,
                    val:
                    i32) {
        let tmp = *field as i64 + val as i64;
        if tmp < 0 {
            {
                ::std::rt::begin_panic("attempt to set negative had count: {tmp}");
            };
        }
        if tmp > u32::MAX as i64 {
            {
                ::std::rt::begin_panic("attempt to overflow had count: {tmp}");
            };
        }
        *field = tmp as u32;
    }
    fn inc(&mut self, key: PieceEnum,
           val:
           i32) {
        use PieceEnum::*;
        match key {
            Empty => { ::std::rt::begin_panic("had in not \'Empty\'"); }
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
type TBoard = [[Piece; 9]; 9];
struct Board {
    board: TBoard,
    had_down: Had,
    had_up: Had,
}
impl Default for Board {
    fn default() -> Self { Board::empty() }
}
impl Clone for Board {
    fn clone(&self)
             ->
             Self {
        Board {
            board: self.board,
            had_down: self.had_down,
            had_up: self.had_up,
        }
    }
}
impl Board {
    fn new() -> Self { Self::normal() }
    fn empty()
        ->
        Self {
        Board {
            board: [[Piece::default(); 9]; 9],
            had_down: Had::default(),
            had_up: Had::default(),
        }
    }
    const fn normal()
        ->
        Self {
        use PieceEnum::*;
        Board {
            board:
            [Piece::new_b([Spear, Knight, Silver, Gold, King, Gold,
                              Silver, Knight, Spear], false),
                Piece::new_b([Empty, Rook, Empty, Empty, Empty, Empty,
                                 Empty, Bishop, Empty], false),
                [Piece::new(Pawn, false, false); 9], [Piece::DEFAULT; 9],
                [Piece::DEFAULT; 9], [Piece::DEFAULT; 9],
                [Piece::new(Pawn, true, false); 9],
                Piece::new_b([Empty, Rook, Empty, Empty, Empty, Empty,
                                 Empty, Bishop, Empty], true),
                Piece::new_b([Spear, Knight, Silver, Gold, King, Gold,
                                 Silver, Knight, Spear], true)],
            had_down: Had::DEFAULT,
            had_up: Had::DEFAULT,
        }
    }
    fn next(&mut self, m: &'_ Move)
            ->
            Self {
        let to_mp = m.to.to_mp();
        let from_mp = m.from;
        let captured = self.get(to_mp).piece;
        let moving = self.get(from_mp);
        self.set(to_mp, moving);
        self.set(from_mp, Piece::default());
        if let PieceEnum::Empty = captured {} else {
            if moving.is_down {
                self.had_down.inc(captured, 1);
            } else { self.had_up.inc(captured, 1); }
        }
        Board {
            board: self.board,
            had_down: self.had_down,
            had_up: self.had_up,
        }
    }
    const fn get(&self, pos: MovePos)
                 ->
                 Piece {
        match pos {
            MovePos::Board(xy) => self.board[xy.x][xy.y],
            MovePos::Had(p) => p,
        }
    }
    fn set(&mut self, pos: MovePos,
           item:
           Piece) {
        match pos {
            MovePos::Board(xy) => { self.board[xy.x][xy.y] = item; }
            MovePos::Had(p) => {
                let val =
                    if let PieceEnum::Empty = item.piece { -1 } else { 1 };
                if p.is_down {
                    self.had_down.inc(p.piece, val);
                } else { self.had_up.inc(p.piece, val); }
            }
        }
    }
    fn search(&self, pat: search::KifPat) -> bool { pat.search(self) }
}
impl From<TBoard> for Board {
    fn from(board: TBoard) -> Self { Self { board, ..Default::default() } }
}
type TKif = Vec<Move>;
struct Kif {
    kif: TKif,
    board: Board,
    move_index: usize,
}
impl Kif {
    ///from_vec for test
    fn t_from_vec(kif: TKif)
                  -> Self { Self { kif, board: Board::normal(), move_index: 0 } }
    fn with_board(kif: TKif, board: Board)
                  -> Self { Self { kif, board, move_index: 0 } }
    fn new(path: &'_ str)
           ->
           Result<Self,
               Box<dyn std::error::Error>> {
        let (_options, moves) =
            match branch(reader::read_kif(path, &reader::Opt::default())) {
                Break { 0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
                Continue { 0: val } => #[allow(unreachable_code)]
                val,
            };
        Ok(Self::t_from_vec(moves))
    }
    fn search(&self, pat: search::KifPat) -> bool { pat.search(&self.board) }
    fn search_all(&self, pat: search::KifPat)
                  ->
                  Option<usize> {
        let mut temp = self.board.clone();
        if pat.search(&temp) { return Some(0); }
        {
            let _t =
                match into_iter(self.kif.iter().enumerate()) {
                    mut iter =>
                        loop {
                            match next(&mut iter) {
                                None {} => break,
                                Some { 0: (i, m) } => {
                                    if pat.search(&temp.next(m)) { return Some(i); }
                                }
                            }
                        },
                };
            _t
        };
        None
    }
    fn next(&mut self)
            ->
            Option<Board> {
        self.move_index += 1;
        Some(self.board.next(match branch(self.kif.get(self.move_index - 1)) {
            Break { 0: residual } => #[allow(unreachable_code)]
            return from_residual(residual),
            Continue { 0: val } => #[allow(unreachable_code)]
            val,
        }))
    }
    fn get_from_index(&self, index: usize)
                      ->
                      Board {
        let mut temp = Board::normal();
        {
            let _t =
                match into_iter(self.kif.iter().take(index)) {
                    mut iter =>
                        loop {
                            match next(&mut iter) {
                                None {} => break,
                                Some { 0: m } => { temp.next(m); }
                            }
                        },
                };
            _t
        };
        temp
    }
}