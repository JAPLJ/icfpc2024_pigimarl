import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *


x = Var("x")
f = Var("f")
# fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))
fix = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


def fix1(f):
    return Lambda(x, x(x))(Lambda(x, f(x(x))))


F = Var("F")
n = Var("n")
DS = Var("D")

rn = Var("r")
dn = Var("d")
ln = Var("l")
un = Var("u")


def next_f(F, rn, dn, ln, un):
    ts = [rn, F, ln, dn, F, un, ln, F, rn, un, F, dn]
    res = ts[0]
    for t in ts[1:]:
        res = res.concat(t)
    return res


rec = fundef(
    (f, F, n, rn, dn, ln, un),
    If(
        n == Int(128),
        F,
        f(next_f(F, rn, dn, ln, un))(n * Int(2))(rn.concat(rn))(dn.concat(dn))(ln.concat(ln))(
            un.concat(un)
        ),
    ),
)
rec = fix1(rec)

res = rec(Str(""))(Int(1))(Str("R"))(Str("D"))(Str("L"))(Str("U"))
ans = Str("solve lambdaman19 ").concat(res)
print(ans)
