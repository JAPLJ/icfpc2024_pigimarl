use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Str(String),
    Lambda(u64, Box<Absyn>),
    Thunk(Rc<RefCell<Thunk>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Thunk {
    Value(Box<Val>),
    Thunk(Box<Absyn>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UOp {
    Neg,
    Not,
    StrToInt,
    IntToStr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Gt,
    Eq,
    LOr,
    LAnd,
    Concat,
    Take,
    Drop,
    Apply,
    LazyApply,
    StrictApply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Absyn {
    Value(Val),
    UnaryOp(UOp, Box<Absyn>),
    BinOp(BOp, Box<Absyn>, Box<Absyn>),
    If(Box<Absyn>, Box<Absyn>, Box<Absyn>),
    Lambda(u64, Box<Absyn>),
    Var(u64),
}

impl Absyn {
    pub fn pretty(&self) -> String {
        let mut s = String::new();
        self.pretty_aux(&mut s, 0);
        s
    }

    fn pretty_aux(&self, s: &mut String, indent: usize) {
        match self {
            Absyn::Value(v) => {
                s.push_str(&format!("{}", v));
            }
            Absyn::UnaryOp(op, a) => {
                s.push_str(&format!("({} ", op));
                a.pretty_aux(s, indent);
                s.push_str(")");
            }
            Absyn::BinOp(op, a, b) => {
                if *op == BOp::Apply || *op == BOp::LazyApply || *op == BOp::StrictApply {
                    s.push_str(&format!("("));
                    a.pretty_aux(s, indent);
                    s.push_str(" ");
                    b.pretty_aux(s, indent);
                    s.push_str(")");
                } else {
                    s.push_str("(");
                    a.pretty_aux(s, indent);
                    s.push_str(&format!(" {} ", op));
                    b.pretty_aux(s, indent);
                    s.push_str(")");
                }
            }
            Absyn::If(a, b, c) => {
                s.push_str(&format!("(if "));
                a.pretty_aux(s, indent);
                s.push_str(" then ");
                b.pretty_aux(s, indent);
                s.push_str(" else ");
                c.pretty_aux(s, indent);
                s.push_str(")");
            }
            Absyn::Lambda(v, e) => {
                s.push_str(&format!("(\\v{} -> ", *v));
                e.pretty_aux(s, indent);
                s.push_str(")");
            }
            Absyn::Var(v) => {
                s.push_str(&format!("v{}", v));
            }
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Bool(b) => write!(f, "{}", b),
            Val::Int(n) => write!(f, "{}", n),
            Val::Str(s) => write!(f, "{}", s),
            Val::Lambda(v, _) => write!(f, "<lambda {}>", v),
            Val::Thunk(_) => write!(f, "<thunk>"),
        }
    }
}

impl fmt::Display for UOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UOp::Neg => write!(f, "-"),
            UOp::Not => write!(f, "!"),
            UOp::StrToInt => write!(f, "#"),
            UOp::IntToStr => write!(f, "$"),
        }
    }
}

impl fmt::Display for BOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BOp::Add => write!(f, "+"),
            BOp::Sub => write!(f, "-"),
            BOp::Mul => write!(f, "*"),
            BOp::Div => write!(f, "/"),
            BOp::Mod => write!(f, "%"),
            BOp::Lt => write!(f, "<"),
            BOp::Gt => write!(f, ">"),
            BOp::Eq => write!(f, "="),
            BOp::LOr => write!(f, "|"),
            BOp::LAnd => write!(f, "&"),
            BOp::Concat => write!(f, "."),
            BOp::Take => write!(f, "T"),
            BOp::Drop => write!(f, "D"),
            BOp::Apply => write!(f, "$"),
            BOp::LazyApply => write!(f, "~"),
            BOp::StrictApply => write!(f, "!"),
        }
    }
}

impl fmt::Display for Absyn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Absyn::Value(v) => write!(f, "{}", v),
            Absyn::UnaryOp(op, a) => write!(f, "({}{})", op, a),
            Absyn::BinOp(op, a, b) => {
                if *op == BOp::Apply {
                    write!(f, "({} {})", a, b)
                } else {
                    write!(f, "({} {} {})", a, op, b)
                }
            }
            Absyn::If(a, b, c) => write!(f, "(if {} then {} else {})", a, b, c),
            Absyn::Lambda(v, e) => write!(f, "(\\{} -> {})", (*v as u8 + b'A') as char, e),
            Absyn::Var(v) => write!(f, "{}", (*v as u8 + b'A') as char),
        }
    }
}
