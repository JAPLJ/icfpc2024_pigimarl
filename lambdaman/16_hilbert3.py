import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *


x = Var("x")
f = Var("f")
F = Var("F")
fix_fn = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


s = Var("s")
c = Var("c")
rot = Var("T")
n_rot = fundef(
    (f, s),
    If(
        s == Str(""),
        Str(""),
        f(s.drop(Int(1))).concat(
            fundef(
                (c,),
                If(
                    c == Str("R"),
                    Str("D"),
                    If(c == Str("D"), Str("L"), If(c == Str("L"), Str("U"), Str("R"))),
                ),
            )(s.take(Int(1)))
        ),
    ),
)


def next_f(rot, prv):
    return (
        rot(prv)
        .concat(Str("DD"))
        .concat(prv)
        .concat(Str("RR"))
        .concat(prv)
        .concat(Str("UU"))
        .concat(rot(rot(rot(prv))))
    )


n = Var("n")
prv = Var("p")
n_solve = fundef(
    (f, rot, prv, n),
    If(n == Int(6), prv, f(rot)(next_f(rot, prv))(n + Int(1))),
)


main = fundef((F,), F(n_solve)(F(n_rot))(Str(""))(Int(0)))
res = main(fix_fn)
# res = fix_fn(n_rot)(Str("RDLU"))
ans = Str("solve lambdaman16 ").concat(res)
print(ans)
