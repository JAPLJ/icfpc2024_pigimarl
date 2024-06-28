use crate::ast::{Absyn, BOp, UOp, Val};
use anyhow::{bail, Result};
use rpds::HashTrieMap;

type Env = HashTrieMap<u64, Box<Absyn>>;

pub fn evaluate(ast: &Absyn) -> Result<(Val, u64)> {
    eval(ast, Env::new())
}

fn eval(ast: &Absyn, env: Env) -> Result<(Val, u64)> {
    match ast {
        Absyn::Value(v) => Ok((v.clone(), 0)),
        Absyn::UnaryOp(op, a) => {
            let (v, c) = eval(a, env)?;
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
                let (va, ca) = eval(a, env.clone())?;
                if let Val::Lambda(v, body) = va {
                    let env = env.insert(v, b.clone());
                    let (r, c) = eval(&body, env)?;
                    Ok((r, ca + c + 1))
                } else {
                    bail!("[Apply] unexpected value: {:?}", a)
                }
            } else {
                let (va, ca) = eval(a, env.clone())?;
                let (vb, cb) = eval(b, env)?;
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
            let (va, ca) = eval(a, env.clone())?;
            if let Val::Bool(cond) = va {
                if cond {
                    let (vb, cb) = eval(b, env)?;
                    Ok((vb, ca + cb))
                } else {
                    let (vc, cc) = eval(c, env)?;
                    Ok((vc, ca + cc))
                }
            } else {
                bail!("[If] unexpected value: {:?}", va)
            }
        }
        Absyn::Lambda(x, body) => {
            let body = subst(body, env)?;
            Ok((Val::Lambda(*x, Box::new(body)), 0))
        }
        Absyn::Var(v) => {
            if let Some(e) = env.clone().get(v) {
                eval(e, env)
            } else {
                bail!("undefined variable: {:?}", v)
            }
        }
    }
}

fn subst(e: &Absyn, env: Env) -> Result<Absyn> {
    match e {
        Absyn::Value(_) => Ok(e.clone()),
        Absyn::UnaryOp(op, a) => {
            let a = subst(a, env)?;
            Ok(Absyn::UnaryOp(*op, Box::new(a)))
        }
        Absyn::BinOp(op, a, b) => {
            let a = subst(a, env.clone())?;
            let b = subst(b, env)?;
            Ok(Absyn::BinOp(*op, Box::new(a), Box::new(b)))
        }
        Absyn::If(a, b, c) => {
            let a = subst(a, env.clone())?;
            let b = subst(b, env.clone())?;
            let c = subst(c, env)?;
            Ok(Absyn::If(Box::new(a), Box::new(b), Box::new(c)))
        }
        Absyn::Lambda(x, body) => {
            let body = subst(body, env)?;
            Ok(Absyn::Lambda(*x, Box::new(body)))
        }
        Absyn::Var(v) => {
            if let Some(e) = env.clone().get(v) {
                subst(e, env)
            } else {
                Ok(Absyn::Var(*v))
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
        eprintln!("{:?}", ast);
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
}
