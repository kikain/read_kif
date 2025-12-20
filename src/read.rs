use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use super::*;
pub struct Opt{
    pub sep:&'static str,
    pub read:Vec<&'static str>
}impl Default for Opt {
    fn default() -> Self {
        Opt {
            sep:"：",
            read:Vec::new()
        }
    }
}impl Opt {
    fn open_all(self) -> (&'static str,Vec<&'static str>) {
        (self.sep,self.read)
    }
}

fn sep_move(m_in:&str) -> Option<Move> {
    let mut _m:Vec<char> = m_in.chars().collect();
    _m = _m[3..].to_vec();
    match _m[0]{
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
    if _m[0] == '▲' || _m[0] == '△'{
        let mut _m: Vec<char> = _m[1..].to_vec();
    }
    let to:Pos;
    {
        let x:usize = match _m[0]{
            '１' => 0,
            '２' => 1,
            '３' => 2,
            '４' => 3,
            '５' => 4,
            '６' => 5,
            '７' => 6,
            '８' => 7,
            '９' => 8,
            c => {
                if let Some(x) = c.to_digit(10) {
                    x as usize
                }else {
                    panic!("{}",c)
                }
            }
        };
        let y:usize = match _m[1]{
            '一' => 1,
            '二' => 2,
            '三' => 3,
            '四' => 4,
            '五' => 5,
            '六' => 6,
            '七' => 7,
            '八' => 8,
            '九' => 9,
            c => {
                if let Some(x) = c.to_digit(10){
                    x as usize
                }else {
                    panic!("char: {}",c)
                }
            }
        };
        to = Pos::new(x, y)
    }
    let _do_promot = false;
    if _m[3] == '成'{
        let _do_promot = true;
    //}else if _m[3] == '打' {
    //    let from = MovePos::Hased(());
    }
    let m:Vec<char> = _m[(if _m[2] == '(' {2} else {3})..].to_vec();
    let from:MovePos;
    {
        let x:usize = match m[1].to_digit(10){
            Some(x @ 1..=9) => (x-1) as usize,
            e => panic!("{:?}",e)
        };
        let y:usize = match m[2].to_digit(10){
            Some(y @ 1..=9) => (y-1) as usize,
            e => panic!("{:?}",e)
        };
        from = MovePos::Board(Pos::new(x, y));
    }
    Some(Move { from, to, do_promot:_do_promot })
}

const MOVES_PREV:&str = "手数----指手---------消費時間--";

pub(crate) fn read_kif(path:&str,opt:Opt) -> Result<(HashMap<String,String>,TKif),Box<dyn std::error::Error>> {
    let (separator,read_sect) = opt.open_all();
    let mut ret:HashMap<String,String> = HashMap::new();
    let mut it = BufReader::new(File::open(path)?).lines();
    let mut last_line:String = String::from("pass");
    'out: for l_res in &mut it {
        let l = l_res?;
        if l.starts_with(MOVES_PREV) { break 'out; }
        if l.trim_start().starts_with("1") { last_line = l; break 'out; }
        if l.starts_with("#") { continue 'out; }
        let mut is_matched = false;
        for sect in read_sect.iter(){ if l.starts_with(sect){ is_matched = true; break; } }
        if ! is_matched { continue 'out; }
        let mut it1 = l.split(separator);
        ret.insert(
            it1.next().unwrap().to_string(),
            it1.next().unwrap().to_string()
        );
    }
    let mut moves:TKif = Vec::new();
    if last_line != "pass" {
        let _1 = sep_move(&last_line.trim_start());
        match _1 {
            Some(val) => {moves.push(val);},
            None => {return Ok((ret,moves));}
        }
    }
    for l_res in it {
        let l = l_res?;
        match sep_move(&l.trim_start()) {
            Some(val) => {moves.push(val);},
            None => {return Ok((ret,moves));}
        }
    }
    Ok((ret,moves))
}