#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Str(String),
    Lambda(u64, Box<Absyn>),
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
