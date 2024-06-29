use anyhow::{bail, Result};
use rpds::HashTrieMap;

use crate::ast::{Absyn, BOp, UOp, Val};

type VName = HashTrieMap<u64, u64>;

struct Parser<'a> {
    toks: Vec<&'a [u8]>,
    alpha: u64,
}

pub fn parse(input: &str) -> Result<Absyn> {
    let toks = input
        .trim()
        .split_whitespace()
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();
    let mut parser = Parser { toks, alpha: 0 };
    Ok(parser.parse()?)
}

fn parse_int(s: &[u8]) -> Result<u64> {
    let mut n = 0;
    for i in 0..s.len() {
        n = n * 94 + (s[i] - b'!') as u64;
    }
    Ok(n)
}

pub fn parse_str(s: &[u8]) -> Result<String> {
    const MAP: &[u8] = concat!(r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ "##, '\n').as_bytes();
    let mut chs = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        chs.push(MAP[(s[i] - b'!') as usize]);
    }
    Ok(String::from_utf8(chs)?)
}

pub fn invert_str(s: &str) -> String {
    const MAP: &[u8] = concat!(r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ "##, '\n').as_bytes();
    let mut chs = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        chs.push(MAP.iter().position(|&c| c == s.as_bytes()[i]).unwrap() as u8 + b'!');
    }
    String::from_utf8(chs).unwrap()
}

pub fn str_from_int(n: u64) -> String {
    let mut s = Vec::new();
    let mut n = n;
    while n > 0 {
        s.push((n % 94) as u8 + b'!');
        n /= 94;
    }
    s.reverse();
    String::from_utf8(s).unwrap()
}

fn parse_uop(c: u8) -> Result<UOp> {
    match c {
        b'-' => Ok(UOp::Neg),
        b'!' => Ok(UOp::Not),
        b'#' => Ok(UOp::StrToInt),
        b'$' => Ok(UOp::IntToStr),
        _ => bail!("unexpected unary operator: {}", c as char),
    }
}

fn parse_bop(c: u8) -> Result<BOp> {
    match c {
        b'+' => Ok(BOp::Add),
        b'-' => Ok(BOp::Sub),
        b'*' => Ok(BOp::Mul),
        b'/' => Ok(BOp::Div),
        b'%' => Ok(BOp::Mod),
        b'<' => Ok(BOp::Lt),
        b'>' => Ok(BOp::Gt),
        b'=' => Ok(BOp::Eq),
        b'|' => Ok(BOp::LOr),
        b'&' => Ok(BOp::LAnd),
        b'.' => Ok(BOp::Concat),
        b'T' => Ok(BOp::Take),
        b'D' => Ok(BOp::Drop),
        b'$' => Ok(BOp::Apply),
        b'~' => Ok(BOp::LazyApply),
        b'!' => Ok(BOp::StrictApply),
        _ => bail!("unexpected binary operator: {}", c as char),
    }
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Result<Absyn> {
        let vn = VName::new();
        let (e, s) = self.inner(vn, 0)?;
        if s != self.toks.len() {
            bail!("unexpected tokens left");
        }
        Ok(e)
    }

    fn inner(&mut self, vn: VName, s: usize) -> Result<(Absyn, usize)> {
        let ind = self.toks[s][0];
        let tok = self.toks[s];
        match ind {
            b'T' => Ok((Absyn::Value(Val::Bool(true)), s + 1)),
            b'F' => Ok((Absyn::Value(Val::Bool(false)), s + 1)),
            b'I' => Ok((Absyn::Value(Val::Int(parse_int(&tok[1..])? as i64)), s + 1)),
            b'S' => Ok((Absyn::Value(Val::Str(parse_str(&tok[1..])?)), s + 1)),
            b'U' => {
                let op = parse_uop(tok[1])?;
                let (e, ss) = self.inner(vn, s + 1)?;
                Ok((Absyn::UnaryOp(op, Box::new(e)), ss))
            }
            b'B' => {
                let op = parse_bop(tok[1])?;
                let (e1, s1) = self.inner(vn.clone(), s + 1)?;
                let (e2, s2) = self.inner(vn.clone(), s1)?;
                Ok((Absyn::BinOp(op, Box::new(e1), Box::new(e2)), s2))
            }
            b'?' => {
                let (e1, s1) = self.inner(vn.clone(), s + 1)?;
                let (e2, s2) = self.inner(vn.clone(), s1)?;
                let (e3, s3) = self.inner(vn.clone(), s2)?;
                Ok((Absyn::If(Box::new(e1), Box::new(e2), Box::new(e3)), s3))
            }
            b'L' => {
                let n = parse_int(&tok[1..])?;
                let nn = self.alpha;
                self.alpha += 1;
                let (e, ss) = self.inner(vn.insert(n, nn), s + 1)?;
                Ok((Absyn::Lambda(nn, Box::new(e)), ss))
            }
            b'v' => {
                let n = parse_int(&tok[1..])?;
                if let Some(nn) = vn.get(&n) {
                    Ok((Absyn::Var(*nn), s + 1))
                } else {
                    // (completely) free variable
                    Ok((Absyn::Var(u64::MAX), s + 1))
                    // bail!("variable not found: {}", n);
                }
            }
            _ => bail!("unexpected token: {}", ind as char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int(b"!").unwrap(), 0);
        assert_eq!(parse_int(b"\"").unwrap(), 1);
        assert_eq!(parse_int(b"/6").unwrap(), 1337);
    }

    #[test]
    fn test_str_from_int() {
        assert_eq!(str_from_int(15818151), "test");
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str(b"B%,,/}Q/2,$_").unwrap(), "Hello World!");
    }

    #[test]
    fn test_complicated() {
        let prog = r##"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"##;
        let ast = parse(prog).unwrap();
        eprintln!("{}", ast);
    }
}
