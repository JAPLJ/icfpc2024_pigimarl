use crate::ast::{Absyn, BOp, UOp, Val};
use anyhow::{bail, Result};
use rpds::HashTrieMap;

type Env = HashTrieMap<u64, u64>;

pub fn evaluate(ast: &Absyn) -> Result<(Val, u64)> {
    let mut evaluator = Evaluator {
        limit: u64::MAX,
        alpha: max_var_id(ast) + 1,
    };
    evaluator.eval(ast)
}

fn max_var_id(ast: &Absyn) -> u64 {
    match ast {
        Absyn::Value(_) => 0,
        Absyn::UnaryOp(_, a) => max_var_id(a),
        Absyn::BinOp(_, a, b) => max_var_id(a).max(max_var_id(b)),
        Absyn::If(a, b, c) => max_var_id(a).max(max_var_id(b)).max(max_var_id(c)),
        Absyn::Lambda(x, body) => (*x).max(max_var_id(body)),
        Absyn::Var(v) => {
            if *v == u64::MAX {
                0
            } else {
                *v
            }
        }
    }
}

struct Evaluator {
    limit: u64,
    alpha: u64,
}

impl Evaluator {
    fn eval(&mut self, ast: &Absyn) -> Result<(Val, u64)> {
        if self.limit == 0 {
            bail!("evaluation limit exceeded");
        }
        self.limit -= 1;
        match ast {
            Absyn::Value(v) => Ok((v.clone(), 0)),
            Absyn::UnaryOp(op, a) => {
                let (v, c) = self.eval(a)?;
                match op {
                    UOp::Neg => {
                        if let Val::Int(n) = v {
                            Ok((Val::Int(-n), c))
                        } else {
                            bail!("[Neg] unexpected value: {:?}", v)
                        }
                    }
                    UOp::Not => {
                        if let Val::Bool(b) = v {
                            Ok((Val::Bool(!b), c))
                        } else {
                            bail!("[Not] unexpected value: {:?}", v)
                        }
                    }
                    UOp::StrToInt => {
                        if let Val::Str(s) = v {
                            let n = s.parse::<i64>()?;
                            Ok((Val::Int(n), c))
                        } else {
                            bail!("[StrToInt] unexpected value: {:?}", v)
                        }
                    }
                    UOp::IntToStr => {
                        if let Val::Int(n) = v {
                            Ok((Val::Str(n.to_string()), c))
                        } else {
                            bail!("[IntToStr] unexpected value: {:?}", v)
                        }
                    }
                }
            }
            Absyn::BinOp(op, a, b) => {
                if *op == BOp::Apply {
                    let (va, ca) = self.eval(a)?;
                    if let Val::Lambda(v, body) = va {
                        let bbody = self.subst(&body, v, b);
                        let (r, c) = self.eval(&bbody)?;
                        Ok((r, ca + c + 1))
                    } else {
                        bail!("[Apply] unexpected value: {:?}", a)
                    }
                } else {
                    let (va, ca) = self.eval(a)?;
                    let (vb, cb) = self.eval(b)?;
                    let c = ca + cb;
                    match op {
                        BOp::Add => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Int(na + nb), c))
                            } else {
                                bail!("[Add] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Sub => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Int(na - nb), c))
                            } else {
                                bail!("[Sub] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Mul => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Int(na * nb), c))
                            } else {
                                bail!("[Mul] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Div => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Int(na / nb), c))
                            } else {
                                bail!("[Div] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Mod => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Int(na % nb), c))
                            } else {
                                bail!("[Mod] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Lt => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Bool(na < nb), c))
                            } else {
                                bail!("[Lt] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Gt => {
                            if let (Val::Int(na), Val::Int(nb)) = (&va, &vb) {
                                Ok((Val::Bool(na > nb), c))
                            } else {
                                bail!("[Gt] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Eq => Ok((Val::Bool(va == vb), c)),
                        BOp::LOr => {
                            if let (Val::Bool(ba), Val::Bool(bb)) = (&va, &vb) {
                                Ok((Val::Bool(*ba || *bb), c))
                            } else {
                                bail!("[LOr] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::LAnd => {
                            if let (Val::Bool(ba), Val::Bool(bb)) = (&va, &vb) {
                                Ok((Val::Bool(*ba && *bb), c))
                            } else {
                                bail!("[LAnd] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Concat => {
                            if let (Val::Str(sa), Val::Str(sb)) = (&va, &vb) {
                                Ok((Val::Str(format!("{}{}", sa, sb)), c))
                            } else {
                                bail!("[Concat] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Take => {
                            if let (Val::Str(s), Val::Int(n)) = (&va, &vb) {
                                Ok((Val::Str(s.chars().take(*n as usize).collect()), c))
                            } else {
                                bail!("[Take] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        BOp::Drop => {
                            if let (Val::Str(s), Val::Int(n)) = (&va, &vb) {
                                Ok((Val::Str(s.chars().skip(*n as usize).collect()), c))
                            } else {
                                bail!("[Drop] unexpected values: {:?}, {:?}", va, vb)
                            }
                        }
                        _ => {
                            bail!("unexpected binary operator: {:?}", op)
                        }
                    }
                }
            }
            Absyn::If(a, b, c) => {
                let (va, ca) = self.eval(a)?;
                if let Val::Bool(cond) = va {
                    if cond {
                        let (vb, cb) = self.eval(b)?;
                        Ok((vb, ca + cb))
                    } else {
                        let (vc, cc) = self.eval(c)?;
                        Ok((vc, ca + cc))
                    }
                } else {
                    bail!("[If] unexpected value: {:?}", va)
                }
            }
            Absyn::Lambda(x, body) => Ok((Val::Lambda(*x, body.clone()), 0)),
            Absyn::Var(v) => {
                bail!("undefined variable: {:?}", v)
            }
        }
    }

    fn subst(&mut self, e: &Absyn, v: u64, b: &Absyn) -> Absyn {
        match e {
            Absyn::Value(v) => Absyn::Value(v.clone()),
            Absyn::UnaryOp(op, a) => Absyn::UnaryOp(*op, Box::new(self.subst(a, v, b))),
            Absyn::BinOp(op, a1, a2) => Absyn::BinOp(
                *op,
                Box::new(self.subst(a1, v, b)),
                Box::new(self.subst(a2, v, b)),
            ),
            Absyn::If(a1, a2, a3) => Absyn::If(
                Box::new(self.subst(a1, v, b)),
                Box::new(self.subst(a2, v, b)),
                Box::new(self.subst(a3, v, b)),
            ),
            Absyn::Lambda(x, body) => Absyn::Lambda(*x, Box::new(self.subst(body, v, b))),
            Absyn::Var(x) => {
                if *x == v {
                    self.rename(b, &Env::new())
                } else {
                    Absyn::Var(*x)
                }
            }
        }
    }

    fn rename(&mut self, e: &Absyn, env: &Env) -> Absyn {
        match e {
            Absyn::Value(v) => Absyn::Value(v.clone()),
            Absyn::UnaryOp(op, a) => Absyn::UnaryOp(*op, Box::new(self.rename(a, env))),
            Absyn::BinOp(op, a1, a2) => Absyn::BinOp(
                *op,
                Box::new(self.rename(a1, env)),
                Box::new(self.rename(a2, env)),
            ),
            Absyn::If(a1, a2, a3) => Absyn::If(
                Box::new(self.rename(a1, env)),
                Box::new(self.rename(a2, env)),
                Box::new(self.rename(a3, env)),
            ),
            Absyn::Lambda(x, body) => {
                let nn = self.alpha;
                self.alpha += 1;
                Absyn::Lambda(nn, Box::new(self.rename(body, &env.insert(*x, nn))))
            }
            Absyn::Var(x) => {
                if let Some(nn) = env.get(x) {
                    Absyn::Var(*nn)
                } else {
                    Absyn::Var(*x)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser;

    use super::*;

    fn parse(s: &str) -> Absyn {
        parser::parse(s).unwrap()
    }

    fn test_eval(prog: &str) -> (Val, u64) {
        let ast = parse(prog);
        let (val, c) = evaluate(&ast).unwrap();
        (val, c)
    }

    #[test]
    fn test_if() {
        let prog = "? B> I# I$ S9%3 S./";
        let (val, c) = test_eval(prog);
        assert_eq!(val, Val::Str("no".to_string()));
        assert_eq!(c, 0);
    }

    #[test]
    fn test_lambda() {
        let prog = "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK";
        let (val, _) = test_eval(prog);
        assert_eq!(val, Val::Str("Hello World!".to_string()));
    }

    #[test]
    fn test_apply() {
        let prog = r##"B$ L# B$ L" B+ v" v" B* I$ I# v8"##;
        let (val, _) = test_eval(prog);
        assert_eq!(val, Val::Int(12));
    }

    #[test]
    fn test_fact() {
        let prog = r##"B$ B$ Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx Lg Ln ? B= vn I! I" B* vn B$ vg B- vn I" I&"##;
        let (val, _) = test_eval(prog);
        assert_eq!(val, Val::Int(120));
    }

    #[test]
    fn test_limit() {
        let prog = r##"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"##;
        let (val, c) = test_eval(prog);
        assert_eq!(val, Val::Int(16));
        assert_eq!(c, 109);
    }
}
